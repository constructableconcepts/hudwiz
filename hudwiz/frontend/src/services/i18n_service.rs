use fluent_templates::{static_loader, LanguageIdentifier, Loader, fluent_bundle::FluentValue};
use std::collections::HashMap;
use std::borrow::Cow;
use std::rc::Rc;
use yew::prelude::*;

static_loader! {
    static LOCALES = {
        locales: "./static/locales",
        fallback_language: "en-US",
    };
}

#[derive(Clone, PartialEq)]
pub struct I18nService {
    lang: LanguageIdentifier,
}

impl I18nService {
    pub fn new(lang: LanguageIdentifier) -> Self {
        Self { lang }
    }

    pub fn tr(&self, text_id: &str) -> String {
        LOCALES.lookup(&self.lang, text_id)
    }

    pub fn tr_with_args(&self, text_id: &str, args: &HashMap<String, String>) -> String {
        let args: HashMap<Cow<str>, FluentValue> = args
            .iter()
            .map(|(k, v)| (Cow::from(k.clone()), v.as_str().into()))
            .collect();
        LOCALES.lookup_with_args(&self.lang, text_id, &args)
    }
}

#[hook]
pub fn use_translation() -> Rc<I18nService> {
    let app_context = use_context::<crate::services::state_service::AppContext>()
        .expect("no app context found for i18n");

    let lang = app_context
        .reducer
        .language
        .parse()
        .unwrap_or_else(|_| "en-US".parse().unwrap());

    Rc::new(I18nService::new(lang))
}