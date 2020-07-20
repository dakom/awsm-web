use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Element, DocumentFragment, HtmlCollection, Document};


pub fn select<T: JsCast>(doc:&Document, query:&str) -> T {
    try_select(doc, query).unwrap_throw()
}

pub fn try_select<T: JsCast>(doc:&Document, query:&str) -> Option<T> {
    doc
        .query_selector(query)
        .unwrap_throw()
        .map(|elem| elem.dyn_into().unwrap_throw())
}

pub fn set_style(elem:&HtmlElement, style:&str, value:&str) {
    try_set_style(elem, style, value).unwrap_throw();
}

pub fn toggle_class(elem:&HtmlElement, class:&str, flag:bool) {
    try_toggle_class(elem, class, flag).unwrap_throw();
}

pub fn try_toggle_class(elem:&HtmlElement, class:&str, flag:bool) -> Result<(), JsValue> {
    let class_list = elem.class_list();
    if flag {
        class_list.add_1(class)
    } else {
        class_list.remove_1(class)
    }
}
pub fn try_set_style(elem:&HtmlElement, style:&str, value:&str) -> Result<(), JsValue> {
    elem.style()
        .set_property(style, value)
}
pub fn try_set_styles(elem:&HtmlElement, styles:&[(&str, &str)]) {
    for (style, value) in styles.iter() {
        let _ = try_set_style(&elem, style, value);
    }
}
pub fn set_styles(elem:&HtmlElement, styles:&[(&str, &str)]) {
    for (style, value) in styles.iter() {
        set_style(&elem, style, value);
    }
}
// gets the parent element via parent id
pub fn append_to_id(doc:&Document, parent_id:&str, fragment:DocumentFragment) {
    let parent:Element = get_element_by_id(&doc, parent_id);
    parent.append_child(&fragment).unwrap_throw();
}
pub fn try_append_to_id(doc:&Document, parent_id:&str, fragment:DocumentFragment) {
    if let Some(parent) = try_get_element_by_id::<Element>(&doc, parent_id) {
        parent.append_child(&fragment).unwrap_throw();
    }
}
// gets the parent element via parent id
pub fn prepend_to_id(doc:&Document, parent_id:&str, fragment:DocumentFragment) {
    let parent:Element = get_element_by_id(&doc, parent_id);
    parent.prepend_with_node_1(&fragment).unwrap_throw();
}
pub fn try_prepend_to_id(doc:&Document, parent_id:&str, fragment:DocumentFragment) {
    if let Some(parent) = try_get_element_by_id::<Element>(&doc, parent_id) {
        parent.prepend_with_node_1(&fragment).unwrap_throw();
    }
}

pub fn try_set_styles_by_id(doc:&Document, id:&str, styles:&[(&str, &str)]) {
    if let Some(elem) = try_get_element_by_id::<HtmlElement>(doc, id) {
        for (style, value) in styles.iter() {
            let _ = try_set_style(&elem, style, value);
        }
    }
}

pub fn try_set_style_by_id(doc:&Document, id:&str, style:&str, value:&str) {
    if let Some(elem) = try_get_element_by_id::<HtmlElement>(doc, id) {
        let _ = try_set_style(&elem, style, value);
    }
}
pub fn set_styles_by_id(doc:&Document, id:&str, styles:&[(&str, &str)]) {
    let elem = get_element_by_id::<HtmlElement>(doc, id);
    for (style, value) in styles.iter() {
        set_style(&elem, style, value);
    }
}

pub fn set_style_by_id(doc:&Document, id:&str, style:&str, value:&str) {
    let elem = get_element_by_id::<HtmlElement>(doc, id);
    set_style(&elem, style, value);
}

pub fn set_children_with_class_styles(parent:&Element, class_names:&str, styles:&[(&str, &str)]) {
    for elem in get_elements_by_class::<HtmlElement>(parent, class_names) {
        for (style, value) in styles.iter() {
            elem.style()
                .set_property(style, value)
                .unwrap_throw();
        }
    }
}

pub fn set_children_with_class_style(parent:&Element, class_names:&str, style:&str, value:&str) {
    for elem in get_elements_by_class::<HtmlElement>(parent, class_names) {
        elem.style()
            .set_property(style, value)
            .unwrap_throw();
    }
}

pub fn get_elements_by_class<T: JsCast>(parent:&Element, class_names:&str) -> Vec<T> {
    let col:HtmlCollection = parent.get_elements_by_class_name(class_names);
    let res = html_collection_to_vec(&col);
    res
}

pub fn get_element_by_id<T: JsCast>(doc:&Document, id:&str) -> T {
    try_get_element_by_id(&doc, &id).unwrap_throw()
}

pub fn try_get_element_by_id<T: JsCast>(doc:&Document, id:&str) -> Option<T> {
    doc.get_element_by_id(id)
        .map(|elem| elem.dyn_into().unwrap_throw())
}

pub fn with_element_id<T: JsCast, F: FnOnce(T)> (doc:&Document, id:&str, f:F) {
    if let Some(elem) = doc.get_element_by_id(id) {
        f(elem.dyn_into().unwrap_throw());
    }
}

pub fn html_collection_to_vec<T: JsCast>(col:&HtmlCollection) -> Vec<T> {
    let mut res:Vec<T> = Vec::with_capacity(col.length() as usize);

    for i in 0..col.length() {
        if let Some(elem) = col.item(i) {
            res.push(elem.dyn_into().unwrap_throw());
        } 
    }

    res
}
