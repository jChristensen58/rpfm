//---------------------------------------------------------------------------//
// Copyright (c) 2017-2019 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted PackFile Manager (RPFM) project,
// which can be found here: https://github.com/Frodo45127/rpfm.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

/*!
Module with all the submodules for controlling the views of each decodeable PackedFile Type.

This module contains the code to manage the views and actions of each decodeable PackedFile View.
!*/

use qt_widgets::widget::Widget;

use core::sync::atomic::{AtomicPtr, Ordering};

use rpfm_lib::packedfile::{DecodedPackedFile, PackedFileType};
use rpfm_lib::packedfile::text::Text;

use crate::CENTRAL_COMMAND;
use crate::ffi::get_text;
use crate::communications::Command;
use crate::utils::create_grid_layout_unsafe;
use self::image::{PackedFileImageView, slots::PackedFileImageViewSlots};
use self::table::{PackedFileTableView, slots::PackedFileTableViewSlots};
use self::text::{PackedFileTextView, slots::PackedFileTextViewSlots};

pub mod image;
pub mod table;
pub mod text;

//-------------------------------------------------------------------------------//
//                              Enums & Structs
//-------------------------------------------------------------------------------//

/// This struct contains the widget of the view of a PackedFile and his info.
pub struct PackedFileView {
    widget: AtomicPtr<Widget>,
    is_preview: bool,
    view: View,
    packed_file_type: PackedFileType,
}

pub enum View {
    Table(PackedFileTableView),
    Text(PackedFileTextView),
    Image(PackedFileImageView),
    //TreeView(AddFromPackFileViewSlots),
    //Decoder(PackedFileDBDecoder),
    //RigidModel(PackedFileRigidModelDataView),
    None,
}

/// One slot to rule them all,
/// One slot to find them,
/// One slot to bring them all
/// and in the darkness bind them.
pub enum TheOneSlot {
    Table(PackedFileTableViewSlots),
    Image(PackedFileImageViewSlots),
    Text(PackedFileTextViewSlots),
    //TreeView(AddFromPackFileViewSlots),
    //Decoder(PackedFileDBDecoder),
    //RigidModel(PackedFileRigidModelDataView),
    None,
}

//-------------------------------------------------------------------------------//
//                             Implementations
//-------------------------------------------------------------------------------//

/// Default implementation for `PackedFileView`.
impl Default for PackedFileView {
    fn default() -> Self {
        let widget = AtomicPtr::new(Widget::new().into_raw());
        create_grid_layout_unsafe(widget.load(Ordering::SeqCst));
        let is_preview = true;
        let view = View::None;
        let packed_file_type = PackedFileType::Unknown;
        Self {
            widget,
            is_preview,
            view,
            packed_file_type,
        }
    }
}

/// Implementation for `PackedFileView`.
impl PackedFileView {

    /// This function returns a mutable pointer to the `Widget` of the `PackedFileView`.
    pub fn get_mut_widget(&self) -> *mut Widget {
        self.widget.load(Ordering::SeqCst)
    }

    /// This function replaces the widget of the `PackedFileView` with the provided one.
    pub fn set_widget(&self, widget: *mut Widget) {
        self.widget.store(widget, Ordering::SeqCst)
    }

    /// This function returns if the `PackedFileView` is a preview or not.
    pub fn get_is_preview(&self) -> bool {
        self.is_preview
    }

    /// This function allows you to set a `PackedFileView` as a preview or normal view.
    pub fn set_is_preview(&mut self, is_preview: bool) {
        self.is_preview = is_preview;
    }

    /// This function returns the view of the specific `PackedFile`.
    pub fn get_view(&self) -> &View {
        &self.view
    }

    /// This function allows you to set an specific View for the `PackedFile`.
    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    /// This function allows you to save a `PackedFileView` to his corresponding `PackedFile`.
    pub fn save(&self, path: &[String]) {

        // This is a two-step process. First, we take the data from the view into a `DecodedPackedFile` format.
        // Then, we send that `DecodedPackedFile` to the backend to replace the older one. We need no response.
        let data = match self.packed_file_type {
            PackedFileType::Image => DecodedPackedFile::Unknown,
            PackedFileType::Text => {
                if let View::Text(view) = self.get_view() {
                    let mut text = Text::default();
                    unsafe { text.set_contents(&get_text(view.get_mut_editor()).to_std_string()) };
                    DecodedPackedFile::Text(text)
                } else { return }
            },
            PackedFileType::Unknown => DecodedPackedFile::Unknown,
            _ => unimplemented!(),
        };

        CENTRAL_COMMAND.send_message_qt(Command::SavePackedFileFromView(path.to_vec(), data));
    }
}
