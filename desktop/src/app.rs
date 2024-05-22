use libhelium::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
struct AuthButton {
    // TODO: stuff
}

#[relm4::factory]
impl FactoryComponent for AuthButton {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    //? libhelium::ContentList
    type ParentWidget = gtk::FlowBox;

    view! {
        #[root]
        libhelium::MiniContentBlock {
            // TODO: impl MiniContentBlock content
        }
    }

    fn init_model(
        value: Self::Init,
        _index: &relm4::factory::DynamicIndex,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub(crate) struct AppModel {
    authfactory: FactoryVecDeque<AuthButton>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    SelectionChanged,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = ();

    view! {
        libhelium::ApplicationWindow {
            set_title: Some("Fyra Authenticator"),
            set_default_width: 400,
            set_default_height: 500,

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 4,
                gtk::ScrolledWindow {
                    #[local_ref]
                    authbox -> gtk::FlowBox {
                        set_selection_mode: gtk::SelectionMode::Single,
                        set_orientation: gtk::Orientation::Vertical,
                        set_vexpand: true,
                        set_hexpand: true,
                        set_valign: gtk::Align::Center,
                        set_halign: gtk::Align::Center,
                        set_min_children_per_line: 1,
                        set_max_children_per_line: 1,
                        set_column_spacing: 4,
                        set_row_spacing: 4,
                        connect_selected_children_changed => AppInput::SelectionChanged,
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let authfactory = FactoryVecDeque::builder()
            .launch(gtk::FlowBox::default())
            .forward(sender.input_sender(), |output| todo!());

        // TODO: populate authfactory
        
        let model = AppModel { authfactory };
        let authbox = model.authfactory.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
