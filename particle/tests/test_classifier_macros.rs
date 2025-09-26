use particle::{
    classes::{ClassClass, Classifier, ClassifierName, HasClassification},
    defn_class, impl_class, impl_has_class,
    properties::Singleton,
};
use pretty_assertions::assert_eq;

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

defn_class! { Component }
impl_class! { Component }

defn_class! { Widget }
impl_class! { Widget : ComponentClass }

#[derive(Clone, Debug, PartialEq)]
pub struct MyWidget {
    title: String,
}

impl_has_class! { MyWidget => WidgetClass }

// ------------------------------------------------------------------------------------------------

#[test]
fn test_component_name() {
    assert_eq!(
        ComponentClass::instance().name(),
        &ClassifierName::new_unchecked("Component"),
    )
}

#[test]
fn test_component_super_class() {
    assert_eq!(
        ComponentClass::instance().super_class(),
        Some(ClassClass::instance()),
    )
}

#[test]
fn test_component_classifier() {
    assert_eq!(
        ComponentClass::instance().classifiers().collect::<Vec<_>>(),
        vec![ClassClass::instance()],
    )
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_widget_name() {
    assert_eq!(
        WidgetClass::instance().name(),
        &ClassifierName::new_unchecked("Widget"),
    )
}

#[test]
fn test_widget_super() {
    assert_eq!(
        WidgetClass::instance().super_class(),
        Some(ComponentClass::instance()),
    )
}

#[test]
fn test_widget_classifier() {
    assert_eq!(
        WidgetClass::instance().classifiers().collect::<Vec<_>>(),
        vec![ClassClass::instance()],
    )
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_mywidget_classifiers() {
    let my_widget = MyWidget {
        title: "This is Cool".to_string(),
    };
    assert_eq!(
        my_widget.classifiers().collect::<Vec<_>>(),
        vec![WidgetClass::instance()],
    )
}
