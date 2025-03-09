#![allow(clippy::comparison_chain)]

use formatter::{Formatter, HtmlFormatter, MarkdownFormatter};
use glib::GString;
use glib_macros::clone;
use libadwaita as adw;
use adw::prelude::*;
use table::Table;

mod formatter;
mod table;

const G_LOG_DOMAIN: &str = "tabela";
const APP_TITLE: &str = "Tabëla";

fn main() {
    static GLIB_LOGGER: glib::GlibLogger = glib::GlibLogger::new(
        glib::GlibLoggerFormat::Plain,
        glib::GlibLoggerDomain::CrateTarget,
    );

    log::set_logger(&GLIB_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let app = adw::Application::builder()
        .application_id("com.mastropaolo.tabela")
        .build();

    app.connect_activate(on_app_activate);
    app.run();
}

fn on_app_activate(app: &adw::Application) {
    let result_buffer = gtk4::TextBuffer::new(None);
    let input_buffer = gtk4::TextBuffer::new(None);

    let result_textbox = gtk4::TextView::with_buffer(&result_buffer);
    result_textbox.set_editable(false);
    result_textbox.set_monospace(true);
    result_textbox.set_wrap_mode(gtk4::WrapMode::Word);

    let input_textbox = gtk4::TextView::with_buffer(&input_buffer);
    input_textbox.set_wrap_mode(gtk4::WrapMode::Word);

    let result_scrollview = gtk4::ScrolledWindow::new();
    result_scrollview.set_child(Some(&result_textbox));
    result_scrollview.set_vexpand(true);
    result_scrollview.set_hexpand(true);

    let input_scrollview = gtk4::ScrolledWindow::new();
    input_scrollview.set_child(Some(&input_textbox));
    input_scrollview.set_vexpand(true);
    input_scrollview.set_hexpand(true);

    let separator_dropdn = gtk4::ComboBoxText::new();
    separator_dropdn.append(Some("optTab"), "Tab");
    separator_dropdn.append(Some("optComma"), "Comma");
    separator_dropdn.append(Some("optSemicolon"), "Semicolon");
    separator_dropdn.set_active_id(Some("optTab"));
    separator_dropdn.set_margin_end(20);

    let output_dropdn = gtk4::ComboBoxText::new();
    output_dropdn.append(Some("optMarkdown"), "Markdown");
    output_dropdn.append(Some("optHtml"), "HTML");
    output_dropdn.set_active_id(Some("optMarkdown"));
    output_dropdn.set_margin_end(20);

    let titles_switch = gtk4::Switch::new();
    titles_switch.set_valign(gtk4::Align::Center);
    titles_switch.set_active(true);
    titles_switch.set_state(true);

    let top_hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    top_hbox.append(&gtk4::Label::new(Some("Separator")));
    top_hbox.append(&separator_dropdn);
    top_hbox.append(&gtk4::Label::new(Some("Output")));
    top_hbox.append(&output_dropdn);
    top_hbox.append(&gtk4::Label::new(Some("Titles")));
    top_hbox.append(&titles_switch);
    top_hbox.set_halign(gtk4::Align::Center);

    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    vbox.set_margin_end(10);
    vbox.set_margin_bottom(10);
    vbox.set_margin_start(10);
    vbox.set_margin_top(10);
    vbox.append(&top_hbox);
    vbox.append(&input_scrollview);
    vbox.append(&result_scrollview);

    let event_handler = clone!(
        #[strong]
        separator_dropdn,
        #[strong]
        output_dropdn,
        #[strong]
        input_buffer,
        #[strong]
        result_buffer,
        #[strong]
        titles_switch,
        move || {
            let formatter = parse_format_option(output_dropdn.active_id());
            let separator = parse_separator_option(separator_dropdn.active_id());
            let text = input_buffer.text(&input_buffer.start_iter(), &input_buffer.end_iter(), true);

            let table = Table::with_text_and_separator(text.as_str(), separator, titles_switch.state());
            let result = formatter.format(table);

            result_buffer.set_text(&result);
        }
    );

    separator_dropdn.connect_changed(clone!(
        #[strong]
        event_handler,
        move |_| {
            event_handler();
        }
    ));
    output_dropdn.connect_changed(clone!(
        #[strong]
        event_handler,
        move |_| {
            event_handler();
        }
    ));
    input_buffer.connect_changed(clone!(
        #[strong]
        event_handler,
        move |_| {
            event_handler();
        }
    ));
    titles_switch.connect_active_notify(clone!(
        #[strong]
        event_handler,
        move |_| {
            event_handler();
        }
    ));

    let header_bar = adw::HeaderBar::new();
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    main_box.append(&header_bar);
    main_box.append(&vbox);

    adw::ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .default_width(800)
        .default_height(600)
        .content(&main_box)
        .build()
        .present();
}

fn parse_separator_option(separator_option: Option<GString>) -> char {
    let Some(separator_option) = separator_option else {
        glib::warn!("Missing selection of separator, assuming TAB");
        return '\t';
    };

    match separator_option.as_str() {
        "optTab" => '\t',
        "optComma" => ',',
        "optSemicolon" => ';',
        s => {
            glib::warn!("Invalid separator {s}, assuming TAB");
            '\t'
        }
    }
}

fn parse_format_option(format_option: Option<GString>) -> Box<dyn Formatter> {
    let Some(format_option) = format_option else {
        glib::warn!("Missing selection of output format, assuming Markdown");
        return Box::new(MarkdownFormatter);
    };

    match format_option.as_str() {
        "optHtml" => Box::new(HtmlFormatter),
        "optMarkdown" => Box::new(MarkdownFormatter),
        "optSemicolon" => Box::new(MarkdownFormatter),
        s => {
            glib::warn!("Invalid output format {s}, assuming Markdown");
            Box::new(MarkdownFormatter)
        }
    }
}

