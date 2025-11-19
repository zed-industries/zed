use crate::merge_from::MergeFrom;

pub use speech_settings_types::SpeechSettings;

impl MergeFrom for SpeechSettings {
    fn merge_from(&mut self, other: &Self) {
        self.enabled.merge_from(&other.enabled);
        self.model.merge_from(&other.model);
        self.ai_provider.merge_from(&other.ai_provider);
    }
}
