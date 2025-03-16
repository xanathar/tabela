/* MIT License
 *
 * Copyright (c) 2025 Marco Mastropaolo
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */

use adw::subclass::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use gettextrs::gettext;

use crate::formatting;
use crate::translatable;

const FORMATTERS: &[(&dyn formatting::Formatter, &str)] = &[
    (&formatting::MarkdownFormatter, translatable("Markdown")),
    (&formatting::HtmlFormatter, translatable("HTML")),
];

const SEPARATORS: &[(char, &str)] = &[
    ('\t', translatable("TAB")),
    (',', translatable("Comma")),
    (';', translatable("Semicolon")),
];

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/mastropaolo/tabela/window.ui")]
    pub struct TabelaWindow {
        #[template_child]
        pub dropdown_separator: TemplateChild<gtk::DropDown>,
        #[template_child]
        pub dropdown_format: TemplateChild<gtk::DropDown>,
        #[template_child]
        pub switch_titles: TemplateChild<gtk::Switch>,
        #[template_child]
        pub text_input: TemplateChild<gtk::TextView>,
        #[template_child]
        pub text_output: TemplateChild<gtk::TextView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TabelaWindow {
        const NAME: &'static str = "TabelaWindow";
        type Type = super::TabelaWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TabelaWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let separators = SEPARATORS.iter().map(|s| gettext(s.1)).collect::<Vec<_>>();
            let separators = gtk::StringList::from_iter(separators);
            self.dropdown_separator.set_model(Some(&separators));

            let formatters = FORMATTERS.iter().map(|s| gettext(s.1)).collect::<Vec<_>>();
            let formatters = gtk::StringList::from_iter(formatters);
            self.dropdown_format.set_model(Some(&formatters));

            self.obj().init();
        }
    }

    impl WidgetImpl for TabelaWindow {}
    impl WindowImpl for TabelaWindow {}
    impl ApplicationWindowImpl for TabelaWindow {}
    impl AdwApplicationWindowImpl for TabelaWindow {}
}

glib::wrapper! {
    pub struct TabelaWindow(ObjectSubclass<imp::TabelaWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl TabelaWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn init(&self) {
        let imp = self.imp();
        let this = self;

        imp.text_input.buffer().connect_changed(clone! {
            #[strong] this,
            move |_| this.compute()
        });
        imp.dropdown_separator.connect_selected_item_notify(clone! {
            #[strong] this,
            move |_| this.compute()
        });
        imp.dropdown_format.connect_selected_item_notify(clone! {
            #[strong] this,
            move |_| this.compute()
        });
        imp.switch_titles.connect_active_notify(clone! {
            #[strong] this,
            move |_| this.compute()
        });
    }

    fn compute(&self) {
        use formatting::Table;

        let imp = self.imp();

        let formatter = Self::parse_format_option(imp.dropdown_format.selected() as usize);
        let separator = Self::parse_separator_option(imp.dropdown_separator.selected() as usize);
        let input_buffer = imp.text_input.buffer();
        let text = input_buffer.text(&input_buffer.start_iter(), &input_buffer.end_iter(), true);

        let table =
            Table::with_text_and_separator(text.as_str(), separator, imp.switch_titles.state());
        let result = formatter.format(table);

        imp.text_output.buffer().set_text(&result);
    }

    fn parse_separator_option(separator_option: usize) -> char {
        if separator_option <= SEPARATORS.len() {
            SEPARATORS[separator_option].0
        } else {
            glib::g_warning!("tabela", "Invalid separator {separator_option}, assuming default");
            SEPARATORS[0].0
        }
    }

    fn parse_format_option(format_option: usize) -> &'static dyn formatting::Formatter {
        if format_option <= FORMATTERS.len() {
            FORMATTERS[format_option].0
        } else {
            glib::g_warning!("tabela", "Invalid format {format_option}, assuming default");
            FORMATTERS[0].0
        }
    }
}
