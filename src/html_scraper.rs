use modular_agent_core::{
    AsModule, Error, ModularAgent, Module, ModuleContext, ModuleData, ModuleOutput, ModuleSpec,
    Result, Value, async_trait, modular_agent,
};
use scraper::{Html, Selector};

static CATEGORY: &str = "Web";

static PORT_HTML: &str = "html";

/// Extract text content from HTML by CSS selector
#[modular_agent(
    title = "HTML Scraper",
    category = CATEGORY,
    inputs = [PORT_HTML],
    outputs = [PORT_HTML],
    string_config(name = "selector"),
)]
struct HtmlScraperModule {
    data: ModuleData,
}

#[async_trait]
impl AsModule for HtmlScraperModule {
    fn new(ma: ModularAgent, id: String, spec: ModuleSpec) -> Result<Self> {
        Ok(Self {
            data: ModuleData::new(ma, id, spec),
        })
    }

    async fn process(&mut self, ctx: ModuleContext, _port: String, value: Value) -> Result<()> {
        let selector_str = self.configs()?.get_string_or_default("selector");
        if selector_str.is_empty() {
            return Ok(());
        }
        let selector = Selector::parse(&selector_str).map_err(|e| {
            Error::InvalidValue(format!("Invalid CSS selector '{}': {}", selector_str, e))
        })?;

        if value.is_array() {
            let mut arr = Vec::new();
            for item in value.as_array().unwrap() {
                let html = item.as_str().ok_or_else(|| {
                    Error::InvalidValue("Input array items for 'html' must be strings".to_string())
                })?;
                let fragment = Html::parse_fragment(html);
                let selected: Vec<Value> = fragment
                    .select(&selector)
                    .map(|elem| Value::string(elem.html()))
                    .collect();
                arr.extend(selected);
            }
            return self.output(ctx, PORT_HTML, Value::array(arr.into())).await;
        }

        let html = value.as_str().ok_or_else(|| {
            Error::InvalidValue("Input value for 'html' must be a string".to_string())
        })?;

        let selected = {
            let document = Html::parse_document(html);
            let selected: Vec<Value> = document
                .select(&selector)
                .map(|elem| Value::string(elem.html()))
                .collect();
            selected.clone()
        };
        self.output(ctx, PORT_HTML, Value::array(selected.into()))
            .await
    }
}
