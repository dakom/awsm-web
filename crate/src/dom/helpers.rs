use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, Element, DocumentFragment, HtmlCollection, Document, NodeList};

pub fn data_id(id:&str) -> String {
    format!("[data-id='{}']", id)
}

pub trait DomSelector {
    fn try_select<T: JsCast>(&self, query:&str) -> Option<T>;
    fn select<T: JsCast>(&self, query:&str) -> T {
        self.try_select(query).unwrap_throw()
    }
    fn select_vec<T: JsCast>(&self, query:&str) -> Vec<T>;
    // gets the parent element via parent id
    fn append_to_data_id(&self, parent_id:&str, fragment:DocumentFragment) {
        let parent:Element = self.select(&data_id(parent_id));
        parent.append_child(&fragment).unwrap_throw();
    }
    fn try_append_to_data_id(&self, parent_id:&str, fragment:DocumentFragment) {
        if let Some(parent) = self.try_select::<Element>(&data_id(parent_id)) {
            parent.append_child(&fragment).unwrap_throw();
        }
    }
    // gets the parent element via parent id
    fn prepend_to_id(&self, parent_id:&str, fragment:DocumentFragment) {
        let parent:Element = self.select(&data_id(parent_id));
        parent.prepend_with_node_1(&fragment).unwrap_throw();
    }
    fn try_prepend_to_id(&self, parent_id:&str, fragment:DocumentFragment) {
        if let Some(parent) = self.try_select::<Element>(&data_id(parent_id)) {
            parent.prepend_with_node_1(&fragment).unwrap_throw();
        }
    }

}

impl DomSelector for Document {
    fn try_select<T: JsCast>(&self, query:&str) -> Option<T> {
        self.query_selector(query)
            .unwrap_throw()
            .map(|elem| elem.dyn_into().unwrap_throw())
    }
    fn select_vec<T: JsCast>(&self, query:&str) -> Vec<T> {
        node_list_to_vec(&self.query_selector_all(query).unwrap_throw())
    }
}
impl DomSelector for Element {
    fn try_select<T: JsCast>(&self, query:&str) -> Option<T> {
        self.query_selector(query)
            .unwrap_throw()
            .map(|elem| elem.dyn_into().unwrap_throw())
    }
    fn select_vec<T: JsCast>(&self, query:&str) -> Vec<T> {
        node_list_to_vec(&self.query_selector_all(query).unwrap_throw())
    }
}

impl DomSelector for DocumentFragment {
    fn try_select<T: JsCast>(&self, query:&str) -> Option<T> {
        self.query_selector(query)
            .unwrap_throw()
            .map(|elem| elem.dyn_into().unwrap_throw())
    }
    fn select_vec<T: JsCast>(&self, query:&str) -> Vec<T> {
        node_list_to_vec(&self.query_selector_all(query).unwrap_throw())
    }
}

pub trait ElementExt {
    fn toggle_class(&self, class:&str, flag:bool) {
        self.try_toggle_class(class, flag).unwrap_throw();
    }
    fn try_toggle_class(&self, class:&str, flag:bool) -> Result<(), JsValue>;
    fn closest_data_id(&self, id:&str) -> Option<Element> {
        self.try_closest_data_id(id).unwrap_throw()
    }
    fn try_closest_data_id(&self, id:&str) -> Result<Option<Element>, JsValue>;
}

impl ElementExt for Element {

    fn try_toggle_class(&self, class:&str, flag:bool) -> Result<(), JsValue> {
        let class_list = self.class_list();
        if flag {
            class_list.add_1(class)
        } else {
            class_list.remove_1(class)
        }
    }
    fn try_closest_data_id(&self, id:&str) -> Result<Option<Element>, JsValue> {
        self.closest(data_id(id))
    }
}

pub trait StyleExt {
    fn set_style(&self, style:&str, value:&str) {
        self.try_set_style(style, value).unwrap_throw();
    }

    fn try_set_style(&self, style:&str, value:&str) -> Result<(), JsValue>;

    fn try_set_styles(&self, styles:&[(&str, &str)]) {
        for (style, value) in styles.iter() {
            let _ = self.try_set_style(style, value);
        }
    }
    fn set_styles(&self, styles:&[(&str, &str)]) {
        for (style, value) in styles.iter() {
            self.set_style(style, value);
        }
    }
}

impl StyleExt for HtmlElement {
    fn try_set_style(&self, style:&str, value:&str) -> Result<(), JsValue> {
        self.style()
            .set_property(style, value)
    }
}

pub trait HtmlCollectionExt {
    fn get_elements_by_class<T: JsCast>(&self, class_names:&str) -> Vec<T>;
    fn set_children_with_class_styles(&self, class_names:&str, styles:&[(&str, &str)]) {
        for elem in self.get_elements_by_class::<HtmlElement>(class_names) {
            for (style, value) in styles.iter() {
                elem.style()
                    .set_property(style, value)
                    .unwrap_throw();
            }
        }
    }
    fn set_children_with_class_style(&self, class_names:&str, style:&str, value:&str) {
        for elem in self.get_elements_by_class::<HtmlElement>(class_names) {
            elem.style()
                .set_property(style, value)
                .unwrap_throw();
        }
    }
}

impl HtmlCollectionExt for Document {
    fn get_elements_by_class<T: JsCast>(&self, class_names:&str) -> Vec<T> {
        let col:HtmlCollection = self.get_elements_by_class_name(class_names);
        let res = html_collection_to_vec(&col);
        res
    }
}
impl HtmlCollectionExt for Element {
    fn get_elements_by_class<T: JsCast>(&self, class_names:&str) -> Vec<T> {
        let col:HtmlCollection = self.get_elements_by_class_name(class_names);
        let res = html_collection_to_vec(&col);
        res
    }
}

trait DocumentExt {
    fn try_get_element_by_id<T: JsCast>(&self, id:&str) -> Option<T>;
    fn get_element_by_id<T: JsCast>(&self, id:&str) -> T {
        self.try_get_element_by_id(&id).unwrap_throw()
    }
    fn with_element_id<T: JsCast, F: FnOnce(T)> (&self, id:&str, f:F) {
        if let Some(elem) = self.try_get_element_by_id::<T>(id) {
            f(elem)
        }
    }
}

impl DocumentExt for Document {
    fn try_get_element_by_id<T: JsCast>(&self, id:&str) -> Option<T> {
        self.get_element_by_id(id)
            .map(|elem| elem.dyn_into().unwrap_throw())
    }
}

impl DocumentExt for DocumentFragment {
    fn try_get_element_by_id<T: JsCast>(&self, id:&str) -> Option<T> {
        self.get_element_by_id(id)
            .map(|elem| elem.dyn_into().unwrap_throw())
    }
}


pub fn node_list_to_vec<T: JsCast>(list:&NodeList) -> Vec<T> {
    let mut res:Vec<T> = Vec::with_capacity(list.length() as usize);

    for i in 0..list.length() {
        if let Some(elem) = list.item(i) {
            res.push(elem.dyn_into().unwrap_throw());
        } 
    }

    res
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
