pub use once_cell;
pub use yazi;

pub use gagalworks_search_macro::load_search_index;
pub use gagalworks_search_shared::*;

pub type LazySearchIndex<R> = once_cell::sync::Lazy<SearchIndex<R>>;
