use modular_agent_core::{
    AsModule, Error, ModularAgent, ModuleContext, ModuleData, ModuleOutput, ModuleSpec, Result,
    Value, async_trait, modular_agent,
};
use reqwest::Client;

static CATEGORY: &str = "Web";

static PORT_URL: &str = "url";
static PORT_TEXT: &str = "text";

/// Fetch text content from a given URL
#[modular_agent(
    title = "Fetch URL",
    category = CATEGORY,
    inputs = [PORT_URL],
    outputs = [PORT_TEXT],
)]
struct FetchUrlModule {
    data: ModuleData,
}

#[async_trait]
impl AsModule for FetchUrlModule {
    fn new(ma: ModularAgent, id: String, spec: ModuleSpec) -> Result<Self> {
        Ok(Self {
            data: ModuleData::new(ma, id, spec),
        })
    }

    async fn process(&mut self, ctx: ModuleContext, _port: String, value: Value) -> Result<()> {
        let url = value.as_str().ok_or_else(|| {
            Error::InvalidValue("Input value for 'url' must be a string".to_string())
        })?;
        // TODO: validate URL

        let client = Client::new();
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::IoError(format!("HTTP Request Error: {}", e)))?;
        let text = response
            .text()
            .await
            .map_err(|e| Error::IoError(format!("HTTP Response Error: {}", e)))?;

        self.output(ctx, PORT_TEXT, Value::string(text)).await
    }
}
