use std::ops::Div;

use gtk::prelude::*;
use ordered_float::NotNan;
use relm4::{
    css,
    gtk,
    typed_view::list::RelmListItem,
    view, RelmWidgetExt,
};
use url::Url;

use crate::common::format::Format;
use crate::common::image;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListItem {
    pub number: u32,
    pub title: String,
    pub description: String,
    pub icon: &'static str,
    pub image: Option<String>,
    pub progress: Option<NotNan<f64>>,
    pub active: bool,
}

pub struct Widgets {
    number: gtk::Label,
    title: gtk::Label,
    description: gtk::Label,
    icon: gtk::Image,
    thumbnail: gtk::Image,
    progress: gtk::ProgressBar,
    root: gtk::Box,
}

impl RelmListItem for ListItem {
    type Root = gtk::Box;
    type Widgets = Widgets;

    fn setup(_item: &gtk::ListItem) -> (gtk::Box, Widgets) {
        view! {
            root = gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_expand: true,
                set_focusable: true,

                gtk::Box {
                    set_margin_horizontal: 8,
                    set_spacing: 10,
                    set_expand: true,

                    #[name = "thumbnail"]
                    gtk::Image {
                        set_width_request: 100,
                        set_height_request: 56,
                        set_icon_size: gtk::IconSize::Large,
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_valign: gtk::Align::Center,
                        set_hexpand: true,
                        set_spacing: 2,

                        gtk::Box {
                            set_spacing: 4,

                            #[name = "number"]
                            gtk::Label {
                                add_css_class: css::classes::CAPTION,
                                set_width_request: 28,
                            },

                            #[name = "title"]
                            gtk::Label {
                                set_halign: gtk::Align::Start,
                                set_ellipsize: gtk::pango::EllipsizeMode::End,
                                set_single_line_mode: true,
                            },
                        },

                        #[name = "description"]
                        gtk::Label {
                            set_css_classes: &[relm4::css::classes::DIM_LABEL, relm4::css::classes::CAPTION],
                            set_halign: gtk::Align::Start,
                            set_ellipsize: gtk::pango::EllipsizeMode::End,
                            set_single_line_mode: true,
                        },
                    },

                    #[name = "icon"]
                    gtk::Image {
                        set_width_request: 24,
                    },
                },

                #[name = "progress"]
                gtk::ProgressBar {
                    add_css_class: css::classes::OSD,
                    set_valign: gtk::Align::End,
                }
            },
        }

        let root_clone = root.clone();
        let widgets = Widgets {
            number,
            title,
            description,
            icon,
            thumbnail,
            progress,
            root: root_clone,
        };

        (root, widgets)
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        let Widgets {
            number,
            title,
            description,
            icon,
            thumbnail,
            progress,
            root,
        } = widgets;

        // Episode number
        number.set_label(&self.number.to_string());
        number.set_visible(self.number.gt(&0));

        // Title & description
        title.set_label(&self.title);
        description.set_label(&self.description.no_line_breaks());
        description.set_visible(!self.description.is_empty());

        // Action icon (play arrow, external link, etc.)
        icon.set_icon_name(Some(self.icon));

        // Progress bar
        let progress_value = self.progress.map_or(0.0, |progress| progress.div(100.0));
        progress.set_fraction(progress_value);
        progress.set_visible(progress_value > 0.0);

        // Active state highlight
        root.set_css_classes(
            if self.active { &["episode-active"] } else { &[] as &[&str] }
        );

        // Thumbnail — start with placeholder, then try to load
        thumbnail.set_icon_name(Some("video-x-generic-symbolic"));
        if let Some(url_str) = &self.image {
            if !url_str.is_empty() {
                if let Ok(url) = Url::parse(url_str) {
                    let thumb = thumbnail.clone();
                    let url_clone = url.clone();
                    relm4::spawn_local(async move {
                        if let Ok(texture) = image::load_as_texture(url_clone, (200, 112)).await {
                            thumb.set_paintable(Some(&texture));
                        }
                    });
                }
            }
        }
    }
}
