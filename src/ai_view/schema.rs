use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RadioOptionSpec {
    pub value: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatItem {
    pub label: String,
    pub value: String,
    pub change: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum AiComponent {
    #[serde(rename = "heading")]
    Heading { text: String, level: Option<u8> },
    #[serde(rename = "text")]
    Text {
        content: String,
        muted: Option<bool>,
    },
    #[serde(rename = "text_input")]
    TextInput {
        id: String,
        label: Option<String>,
        placeholder: Option<String>,
        required: Option<bool>,
    },
    #[serde(rename = "email_input")]
    EmailInput {
        id: String,
        label: Option<String>,
        placeholder: Option<String>,
    },
    #[serde(rename = "select")]
    Select {
        id: String,
        label: Option<String>,
        options: Vec<String>,
        placeholder: Option<String>,
    },
    #[serde(rename = "radio")]
    Radio {
        id: String,
        label: Option<String>,
        options: Vec<RadioOptionSpec>,
    },
    #[serde(rename = "chip_select")]
    ChipSelect {
        id: String,
        label: Option<String>,
        options: Vec<String>,
        max_selections: Option<usize>,
    },
    #[serde(rename = "checkbox")]
    Checkbox {
        id: String,
        label: String,
        description: Option<String>,
    },
    #[serde(rename = "textarea")]
    Textarea {
        id: String,
        label: Option<String>,
        placeholder: Option<String>,
        rows: Option<u32>,
    },
    #[serde(rename = "score_ring")]
    ScoreRing {
        score: u32,
        size: Option<u32>,
        label: Option<String>,
    },
    #[serde(rename = "card")]
    Card {
        title: Option<String>,
        content: String,
        glow: Option<String>,
    },
    #[serde(rename = "divider")]
    Divider {},
    #[serde(rename = "progress")]
    Progress {
        value: f64,
        max: Option<f64>,
        label: Option<String>,
    },
    #[serde(rename = "chat_message")]
    ChatMessage { sender: String, content: String },
    #[serde(rename = "file_upload")]
    FileUpload {
        id: String,
        accept: Option<String>,
        label: Option<String>,
    },
    #[serde(rename = "stats_row")]
    StatsRow { items: Vec<StatItem> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AiViewSchema {
    pub layout: Option<String>,
    pub components: Vec<AiComponent>,
    pub actions: Option<AiActions>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AiActions {
    pub submit_label: Option<String>,
    pub skip_allowed: Option<bool>,
    pub back_allowed: Option<bool>,
}
