use html_to_markdown_rs::{ConversionOptions, PreprocessingPreset, convert};
use modular_agent_core::{
    AsModule, Error, ModularAgent, ModuleContext, ModuleData, ModuleOutput, ModuleSpec, Result,
    Value, async_trait, modular_agent,
};

static CATEGORY: &str = "Web";

static PORT_HTML: &str = "html";
static PORT_MARKDOWN: &str = "markdown";

/// Convert HTML to Markdown
#[modular_agent(
    title = "HTML to Markdown",
    category = CATEGORY,
    inputs = [PORT_HTML],
    outputs = [PORT_MARKDOWN],
)]
struct HtmlToMarkdownModule {
    data: ModuleData,
}

#[async_trait]
impl AsModule for HtmlToMarkdownModule {
    fn new(ma: ModularAgent, id: String, spec: ModuleSpec) -> Result<Self> {
        Ok(Self {
            data: ModuleData::new(ma, id, spec),
        })
    }

    async fn process(&mut self, ctx: ModuleContext, _port: String, value: Value) -> Result<()> {
        if value.is_array() {
            let mut arr = vec![];
            for item in value.as_array().unwrap() {
                let html = item.as_str().ok_or_else(|| {
                    Error::InvalidValue("Input array items for 'html' must be strings".to_string())
                })?;
                let markdown = html2markdown(html)?;
                arr.push(Value::string(markdown));
            }
            return self
                .output(ctx, PORT_MARKDOWN, Value::array(arr.into()))
                .await;
        }

        let html = value.as_str().ok_or_else(|| {
            Error::InvalidValue("Input value for 'html' must be a string".to_string())
        })?;
        let markdown = html2markdown(html)?;
        self.output(ctx, PORT_MARKDOWN, Value::string(markdown))
            .await
    }
}

fn html2markdown(html: &str) -> Result<String> {
    let mut options = ConversionOptions::default();
    options.preprocessing.enabled = true;
    options.preprocessing.preset = PreprocessingPreset::Aggressive;
    options.preprocessing.remove_navigation = true;
    options.preprocessing.remove_forms = true;
    // Column padding is pure whitespace once the markdown reaches an LLM; a
    // link-heavy table costs ~75% more without this.
    options.compact_tables = true;

    let result = convert(html, Some(options))
        .map_err(|e| Error::InvalidValue(format!("Failed to convert HTML to Markdown: {}", e)))?;
    Ok(result.content.unwrap_or_default())
}
