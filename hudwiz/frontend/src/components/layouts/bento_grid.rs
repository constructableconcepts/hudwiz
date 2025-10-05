use yew::prelude::*;

/// The main container for the Bento Grid layout.
/// It expects `BentoGridItem` components as its children.
#[derive(Properties, Clone, PartialEq)]
pub struct BentoGridProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<BentoGridItem>,
}

#[function_component(BentoGrid)]
pub fn bento_grid(props: &BentoGridProps) -> Html {
    html! {
        <div class="bento-grid">
            { for props.children.iter() }
        </div>
    }
}

/// An individual item to be placed within the `BentoGrid`.
/// It allows for specifying column and row spans to create variably sized grid cells.
#[derive(Properties, Clone, PartialEq)]
pub struct BentoGridItemProps {
    /// The content of the grid item.
    #[prop_or_default]
    pub children: Children,
    /// The number of columns the item should span.
    #[prop_or(1)]
    pub col_span: u32,
    /// The number of rows the item should span.
    #[prop_or(1)]
    pub row_span: u32,
    /// Additional CSS classes to apply to the item.
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(BentoGridItem)]
pub fn bento_grid_item(props: &BentoGridItemProps) -> Html {
    let style = format!(
        "grid-column: span {}; grid-row: span {};",
        props.col_span, props.row_span
    );
    let class = classes!("bento-grid-item", props.class.clone());

    html! {
        <div {class} {style}>
            { for props.children.iter() }
        </div>
    }
}