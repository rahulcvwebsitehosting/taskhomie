use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub api_key_env: String,
    pub models: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub id: String,
    pub label: String,
    pub free: bool,
}

lazy_static::lazy_static! {
    pub static ref PROVIDERS: HashMap<String, ProviderInfo> = {
        let mut m = HashMap::new();

        m.insert("anthropic".to_string(), ProviderInfo {
            id: "anthropic".to_string(),
            name: "Anthropic".to_string(),
            base_url: "https://api.anthropic.com/v1/messages".to_string(),
            api_key_env: "ANTHROPIC_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "claude-haiku-4-5-20251001".to_string(), label: "Haiku 4.5".to_string(), free: false },
                ModelInfo { id: "claude-sonnet-4-5".to_string(), label: "Sonnet 4.5".to_string(), free: false },
                ModelInfo { id: "claude-opus-4-5".to_string(), label: "Opus 4.5".to_string(), free: false },
            ],
        });

        m.insert("nvidia".to_string(), ProviderInfo {
            id: "nvidia".to_string(),
            name: "Nvidia NIM".to_string(),
            base_url: "https://integrate.api.nvidia.com/v1".to_string(),
            api_key_env: "NVIDIA_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "deepseek-ai/deepseek-v4-pro".to_string(), label: "DeepSeek V4 Pro".to_string(), free: true },
                ModelInfo { id: "nvidia/llama-3.3-70b-instruct".to_string(), label: "Llama 3.3 70B".to_string(), free: true },
                ModelInfo { id: "nvidia/llama-3.1-405b-instruct".to_string(), label: "Llama 3.1 405B".to_string(), free: true },
                ModelInfo { id: "zhipuai/glm-4".to_string(), label: "GLM-4".to_string(), free: true },
                ModelInfo { id: "meta/llama-3.1-8b-instruct".to_string(), label: "Llama 3.1 8B".to_string(), free: true },
                ModelInfo { id: "mistralai/mistral-large-2-instruct".to_string(), label: "Mistral Large 2".to_string(), free: true },
                ModelInfo { id: "qwen/qwen2.5-72b-instruct".to_string(), label: "Qwen 2.5 72B".to_string(), free: true },
                ModelInfo { id: "google/gemma-2-27b-it".to_string(), label: "Gemma 2 27B".to_string(), free: true },
            ],
        });

        m.insert("openrouter".to_string(), ProviderInfo {
            id: "openrouter".to_string(),
            name: "OpenRouter".to_string(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            api_key_env: "OPENROUTER_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "meta-llama/llama-3.3-70b-instruct:free".to_string(), label: "Llama 3.3 70B (Free)".to_string(), free: true },
                ModelInfo { id: "mistralai/mistral-7b-instruct:free".to_string(), label: "Mistral 7B (Free)".to_string(), free: true },
                ModelInfo { id: "google/gemma-2-9b-it:free".to_string(), label: "Gemma 2 9B (Free)".to_string(), free: true },
                ModelInfo { id: "qwen/qwen-2.5-72b-instruct:free".to_string(), label: "Qwen 2.5 72B (Free)".to_string(), free: true },
                ModelInfo { id: "deepseek/deepseek-chat:free".to_string(), label: "DeepSeek Chat (Free)".to_string(), free: true },
                ModelInfo { id: "nousresearch/hermes-3-llama-3.1-405b:free".to_string(), label: "Hermes 3 405B (Free)".to_string(), free: true },
                ModelInfo { id: "meta-llama/llama-3.1-8b-instruct:free".to_string(), label: "Llama 3.1 8B (Free)".to_string(), free: true },
                ModelInfo { id: "openai/gpt-4o-mini".to_string(), label: "GPT-4o Mini".to_string(), free: false },
                ModelInfo { id: "anthropic/claude-3.5-sonnet".to_string(), label: "Claude 3.5 Sonnet".to_string(), free: false },
            ],
        });

        m.insert("mistral".to_string(), ProviderInfo {
            id: "mistral".to_string(),
            name: "Mistral".to_string(),
            base_url: "https://api.mistral.ai/v1".to_string(),
            api_key_env: "MISTRAL_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "mistral-large-latest".to_string(), label: "Mistral Large".to_string(), free: true },
                ModelInfo { id: "mistral-small-latest".to_string(), label: "Mistral Small".to_string(), free: true },
                ModelInfo { id: "codestral-latest".to_string(), label: "Codestral".to_string(), free: true },
                ModelInfo { id: "open-mistral-nemo".to_string(), label: "Mistral Nemo".to_string(), free: true },
                ModelInfo { id: "open-mixtral-8x22b".to_string(), label: "Mixtral 8x22B".to_string(), free: true },
                ModelInfo { id: "open-mixtral-8x7b".to_string(), label: "Mixtral 8x7B".to_string(), free: true },
            ],
        });

        m.insert("gemini".to_string(), ProviderInfo {
            id: "gemini".to_string(),
            name: "Google Gemini".to_string(),
            base_url: "https://generativelanguage.googleapis.com/v1beta/openai".to_string(),
            api_key_env: "GEMINI_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "gemini-2.5-flash".to_string(), label: "Gemini 2.5 Flash".to_string(), free: true },
                ModelInfo { id: "gemini-2.5-pro".to_string(), label: "Gemini 2.5 Pro".to_string(), free: true },
                ModelInfo { id: "gemini-1.5-flash".to_string(), label: "Gemini 1.5 Flash".to_string(), free: true },
                ModelInfo { id: "gemini-1.5-pro".to_string(), label: "Gemini 1.5 Pro".to_string(), free: true },
                ModelInfo { id: "gemini-2.0-flash".to_string(), label: "Gemini 2.0 Flash".to_string(), free: true },
                ModelInfo { id: "gemma-3-27b-it".to_string(), label: "Gemma 3 27B".to_string(), free: true },
            ],
        });

        m.insert("groq".to_string(), ProviderInfo {
            id: "groq".to_string(),
            name: "Groq".to_string(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            api_key_env: "GROQ_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "llama-3.3-70b-versatile".to_string(), label: "Llama 3.3 70B".to_string(), free: true },
                ModelInfo { id: "llama-4-scout-17b-16e-instruct".to_string(), label: "Llama 4 Scout".to_string(), free: true },
                ModelInfo { id: "qwen-qwq-32b".to_string(), label: "QwQ 32B".to_string(), free: true },
                ModelInfo { id: "gemma2-9b-it".to_string(), label: "Gemma 2 9B".to_string(), free: true },
                ModelInfo { id: "mixtral-8x7b-32768".to_string(), label: "Mixtral 8x7B".to_string(), free: true },
                ModelInfo { id: "llama-3.1-8b-instant".to_string(), label: "Llama 3.1 8B".to_string(), free: true },
                ModelInfo { id: "llama-3.2-1b-preview".to_string(), label: "Llama 3.2 1B".to_string(), free: true },
            ],
        });

        m.insert("cerebras".to_string(), ProviderInfo {
            id: "cerebras".to_string(),
            name: "Cerebras".to_string(),
            base_url: "https://api.cerebras.ai/v1".to_string(),
            api_key_env: "CEREBRAS_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "llama-3.3-70b".to_string(), label: "Llama 3.3 70B".to_string(), free: true },
                ModelInfo { id: "llama-3.1-8b".to_string(), label: "Llama 3.1 8B".to_string(), free: true },
            ],
        });

        m.insert("together".to_string(), ProviderInfo {
            id: "together".to_string(),
            name: "Together AI".to_string(),
            base_url: "https://api.together.xyz/v1".to_string(),
            api_key_env: "TOGETHER_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo".to_string(), label: "Llama 3.1 70B Turbo".to_string(), free: false },
                ModelInfo { id: "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo".to_string(), label: "Llama 3.1 8B Turbo".to_string(), free: false },
                ModelInfo { id: "mistralai/Mixtral-8x7B-Instruct-v0.1".to_string(), label: "Mixtral 8x7B".to_string(), free: false },
                ModelInfo { id: "Qwen/Qwen2.5-72B-Instruct-Turbo".to_string(), label: "Qwen 2.5 72B Turbo".to_string(), free: false },
                ModelInfo { id: "deepseek-ai/DeepSeek-V3".to_string(), label: "DeepSeek V3".to_string(), free: false },
            ],
        });

        m.insert("deepseek".to_string(), ProviderInfo {
            id: "deepseek".to_string(),
            name: "DeepSeek".to_string(),
            base_url: "https://api.deepseek.com/v1".to_string(),
            api_key_env: "DEEPSEEK_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "deepseek-chat".to_string(), label: "DeepSeek Chat".to_string(), free: false },
                ModelInfo { id: "deepseek-reasoner".to_string(), label: "DeepSeek Reasoner".to_string(), free: false },
            ],
        });

        m.insert("opencode-zen".to_string(), ProviderInfo {
            id: "opencode-zen".to_string(),
            name: "OpenCode Zen".to_string(),
            base_url: "https://api.opencodezen.com/v1".to_string(),
            api_key_env: "OPENCODE_ZEN_API_KEY".to_string(),
            models: vec![
                ModelInfo { id: "zen-1".to_string(), label: "Zen 1".to_string(), free: true },
            ],
        });

        m
    };
}

pub fn get_provider(id: &str) -> Option<&'static ProviderInfo> {
    PROVIDERS.get(id)
}

pub fn get_all_providers() -> Vec<&'static ProviderInfo> {
    PROVIDERS.values().collect()
}

pub fn is_anthropic(provider_id: &str) -> bool {
    provider_id == "anthropic"
}

pub fn get_api_key_for_provider(provider_id: &str) -> Option<String> {
    let provider = get_provider(provider_id)?;
    std::env::var(&provider.api_key_env).ok()
}
