use crate::ItemHandle;
use gpui2::{AnyView, AppContext, EventEmitter, Render, View, ViewContext, WindowContext};

pub trait ToolbarItemView: Render + EventEmitter {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation;

    fn location_for_event(
        &self,
        _event: &Self::Event,
        current_location: ToolbarItemLocation,
        _cx: &AppContext,
    ) -> ToolbarItemLocation {
        current_location
    }

    fn pane_focus_update(&mut self, _pane_focused: bool, _cx: &mut ViewContext<Self>) {}

    /// Number of times toolbar's height will be repeated to get the effective height.
    /// Useful when multiple rows one under each other are needed.
    /// The rows have the same width and act as a whole when reacting to resizes and similar events.
    fn row_count(&self, _cx: &WindowContext) -> usize {
        1
    }
}

trait ToolbarItemViewHandle: Send {
    fn id(&self) -> usize;
    fn to_any(&self) -> AnyView;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    ) -> ToolbarItemLocation;
    fn focus_changed(&mut self, pane_focused: bool, cx: &mut WindowContext);
    fn row_count(&self, cx: &WindowContext) -> usize;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ToolbarItemLocation {
    Hidden,
    PrimaryLeft { flex: Option<(f32, bool)> },
    PrimaryRight { flex: Option<(f32, bool)> },
    Secondary,
}

pub struct Toolbar {
    active_item: Option<Box<dyn ItemHandle>>,
    hidden: bool,
    can_navigate: bool,
    items: Vec<(Box<dyn ToolbarItemViewHandle>, ToolbarItemLocation)>,
}

// todo!()
// impl View for Toolbar {
//     fn ui_name() -> &'static str {
//         "Toolbar"
//     }

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
//         let theme = &theme::current(cx).workspace.toolbar;

//         let mut primary_left_items = Vec::new();
//         let mut primary_right_items = Vec::new();
//         let mut secondary_item = None;
//         let spacing = theme.item_spacing;
//         let mut primary_items_row_count = 1;

//         for (item, position) in &self.items {
//             match *position {
//                 ToolbarItemLocation::Hidden => {}

//                 ToolbarItemLocation::PrimaryLeft { flex } => {
//                     primary_items_row_count = primary_items_row_count.max(item.row_count(cx));
//                     let left_item = ChildView::new(item.as_any(), cx).aligned();
//                     if let Some((flex, expanded)) = flex {
//                         primary_left_items.push(left_item.flex(flex, expanded).into_any());
//                     } else {
//                         primary_left_items.push(left_item.into_any());
//                     }
//                 }

//                 ToolbarItemLocation::PrimaryRight { flex } => {
//                     primary_items_row_count = primary_items_row_count.max(item.row_count(cx));
//                     let right_item = ChildView::new(item.as_any(), cx).aligned().flex_float();
//                     if let Some((flex, expanded)) = flex {
//                         primary_right_items.push(right_item.flex(flex, expanded).into_any());
//                     } else {
//                         primary_right_items.push(right_item.into_any());
//                     }
//                 }

//                 ToolbarItemLocation::Secondary => {
//                     secondary_item = Some(
//                         ChildView::new(item.as_any(), cx)
//                             .constrained()
//                             .with_height(theme.height * item.row_count(cx) as f32)
//                             .into_any(),
//                     );
//                 }
//             }
//         }

//         let container_style = theme.container;
//         let height = theme.height * primary_items_row_count as f32;

//         let mut primary_items = Flex::row().with_spacing(spacing);
//         primary_items.extend(primary_left_items);
//         primary_items.extend(primary_right_items);

//         let mut toolbar = Flex::column();
//         if !primary_items.is_empty() {
//             toolbar.add_child(primary_items.constrained().with_height(height));
//         }
//         if let Some(secondary_item) = secondary_item {
//             toolbar.add_child(secondary_item);
//         }

//         if toolbar.is_empty() {
//             toolbar.into_any_named("toolbar")
//         } else {
//             toolbar
//                 .contained()
//                 .with_style(container_style)
//                 .into_any_named("toolbar")
//         }
//     }
// }

// <<<<<<< HEAD
// =======
// #[allow(clippy::too_many_arguments)]
// fn nav_button<A: Action, F: 'static + Fn(&mut Toolbar, &mut ViewContext<Toolbar>)>(
//     svg_path: &'static str,
//     style: theme::Interactive<theme::IconButton>,
//     nav_button_height: f32,
//     tooltip_style: TooltipStyle,
//     enabled: bool,
//     spacing: f32,
//     on_click: F,
//     tooltip_action: A,
//     action_name: &'static str,
//     cx: &mut ViewContext<Toolbar>,
// ) -> AnyElement<Toolbar> {
//     MouseEventHandler::new::<A, _>(0, cx, |state, _| {
//         let style = if enabled {
//             style.style_for(state)
//         } else {
//             style.disabled_style()
//         };
//         Svg::new(svg_path)
//             .with_color(style.color)
//             .constrained()
//             .with_width(style.icon_width)
//             .aligned()
//             .contained()
//             .with_style(style.container)
//             .constrained()
//             .with_width(style.button_width)
//             .with_height(nav_button_height)
//             .aligned()
//             .top()
//     })
//     .with_cursor_style(if enabled {
//         CursorStyle::PointingHand
//     } else {
//         CursorStyle::default()
//     })
//     .on_click(MouseButton::Left, move |_, toolbar, cx| {
//         on_click(toolbar, cx)
//     })
//     .with_tooltip::<A>(
//         0,
//         action_name,
//         Some(Box::new(tooltip_action)),
//         tooltip_style,
//         cx,
//     )
//     .contained()
//     .with_margin_right(spacing)
//     .into_any_named("nav button")
// }

// >>>>>>> 139cbbfd3aebd0863a7d51b0c12d748764cf0b2e
impl Toolbar {
    pub fn new() -> Self {
        Self {
            active_item: None,
            items: Default::default(),
            hidden: false,
            can_navigate: true,
        }
    }

    pub fn set_can_navigate(&mut self, can_navigate: bool, cx: &mut ViewContext<Self>) {
        self.can_navigate = can_navigate;
        cx.notify();
    }

    pub fn add_item<T>(&mut self, item: View<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + ToolbarItemView,
    {
        let location = item.set_active_pane_item(self.active_item.as_deref(), cx);
        cx.subscribe(&item, |this, item, event, cx| {
            if let Some((_, current_location)) =
                this.items.iter_mut().find(|(i, _)| i.id() == item.id())
            {
                let new_location = item
                    .read(cx)
                    .location_for_event(event, *current_location, cx);
                if new_location != *current_location {
                    *current_location = new_location;
                    cx.notify();
                }
            }
        })
        .detach();
        self.items.push((Box::new(item), location));
        cx.notify();
    }

    pub fn set_active_item(&mut self, item: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        self.active_item = item.map(|item| item.boxed_clone());
        self.hidden = self
            .active_item
            .as_ref()
            .map(|item| !item.show_toolbar(cx))
            .unwrap_or(false);

        for (toolbar_item, current_location) in self.items.iter_mut() {
            let new_location = toolbar_item.set_active_pane_item(item, cx);
            if new_location != *current_location {
                *current_location = new_location;
                cx.notify();
            }
        }
    }

    pub fn focus_changed(&mut self, focused: bool, cx: &mut ViewContext<Self>) {
        for (toolbar_item, _) in self.items.iter_mut() {
            toolbar_item.focus_changed(focused, cx);
        }
    }

    pub fn item_of_type<T: ToolbarItemView>(&self) -> Option<View<T>> {
        self.items
            .iter()
            .find_map(|(item, _)| item.to_any().downcast().ok())
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }
}

impl<T: ToolbarItemView> ToolbarItemViewHandle for View<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn to_any(&self) -> AnyView {
        self.clone().into_any()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    ) -> ToolbarItemLocation {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, cx)
        })
    }

    fn focus_changed(&mut self, pane_focused: bool, cx: &mut WindowContext) {
        self.update(cx, |this, cx| {
            this.pane_focus_update(pane_focused, cx);
            cx.notify();
        });
    }

    fn row_count(&self, cx: &WindowContext) -> usize {
        self.read(cx).row_count(cx)
    }
}

// todo!()
// impl From<&dyn ToolbarItemViewHandle> for AnyViewHandle {
//     fn from(val: &dyn ToolbarItemViewHandle) -> Self {
//         val.as_any().clone()
//     }
// }
