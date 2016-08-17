//! Macros used to ease construction of the API
//! #Descritpion
//! By turning repeated blocks of code into expandable macros, it allows for easily identifiable
//! use cases without having to import the function to call it each time.

/// Paginates a request by modifying the url
macro_rules! paginate {
    ($url:expr, $page_num:expr, $per_page:expr) => (
        if $page_num.is_some() || $per_page.is_some() {
            pagination(&mut $url, $page_num, $per_page);
        }
    );
}
