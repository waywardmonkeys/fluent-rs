use intl_memoizer::{concurrent::IntlLangMemoizer, Memoizable};
use rustc_hash::FxHashMap;
use unic_langid::LanguageIdentifier;

use crate::memoizer::MemoizerKind;
use crate::types::FluentType;

/// Specialized [`FluentBundle`](crate::bundle::FluentBundle) over
/// concurrent [`IntlLangMemoizer`](intl_memoizer::concurrent::IntlLangMemoizer).
///
/// A concurrent `FluentBundle` can be constructed with the
/// [`FluentBundle::new_concurrent`] method.
///
/// See [`FluentBundle`](crate::FluentBundle) for the non-concurrent specialization.
pub type FluentBundle<R> = crate::bundle::FluentBundle<R, IntlLangMemoizer>;

impl<R> FluentBundle<R> {
    /// A constructor analogous to [`FluentBundle::new`] but operating
    /// on a concurrent version of [`IntlLangMemoizer`] over [`Mutex`](std::sync::Mutex).
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::concurrent::FluentBundle;
    /// use fluent_bundle::FluentResource;
    /// use unic_langid::langid;
    ///
    /// let langid_en = langid!("en-US");
    /// let mut bundle: FluentBundle<FluentResource> =
    ///     FluentBundle::new_concurrent(vec![langid_en]);
    /// ```
    pub fn new_concurrent(locales: Vec<LanguageIdentifier>) -> Self {
        let first_locale = locales.get(0).cloned().unwrap_or_default();
        Self {
            locales,
            resources: vec![],
            entries: FxHashMap::default(),
            intls: IntlLangMemoizer::new(first_locale),
            use_isolating: true,
            transform: None,
            formatter: None,
        }
    }
}

impl MemoizerKind for IntlLangMemoizer {
    fn new(lang: LanguageIdentifier) -> Self
    where
        Self: Sized,
    {
        Self::new(lang)
    }

    fn with_try_get_threadsafe<I, R, U>(&self, args: I::Args, cb: U) -> Result<R, I::Error>
    where
        Self: Sized,
        I: Memoizable + Send + Sync + 'static,
        I::Args: Send + Sync + 'static,
        U: FnOnce(&I) -> R,
    {
        self.with_try_get(args, cb)
    }

    fn stringify_value(&self, value: &dyn FluentType) -> std::borrow::Cow<'static, str> {
        value.as_string_threadsafe(self)
    }
}
