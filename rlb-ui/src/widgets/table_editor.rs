use crate::state::{AppState, LoadedFile};
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use rlb_domain::{FieldConstraint, FieldDescriptor, FieldKind, RLBFile, TableId, Value};
use rlb_error::Result;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    let Some(loaded) = &mut state.loaded else {
        ui.centered_and_justified(|ui| ui.weak("Open an .rlb file"));
        return;
    };

    let Some(table_id) = loaded.selected_table else {
        ui.centered_and_justified(|ui| ui.weak("Select a table"));
        return;
    };
    let Some((label, fields, entry_count)) = table_lookup(loaded, table_id).ok().flatten() else {
        ui.centered_and_justified(|ui| ui.weak("Table not resolvable"));
        return;
    };

    ui.heading(label);
    ui.separator();

    let row_height = ui.text_style_height(&egui::TextStyle::Body);

    let LoadedFile { file, dirty, .. } = loaded;
    let mut error = None;

    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .auto_shrink([true, true])
        .columns(Column::auto(), fields.len())
        .header(row_height, |mut header| {
            for field in fields {
                header.col(|ui| {
                    ui.strong(field.name).on_hover_text(field_hint(field));
                });
            }
        })
        .body(|body| {
            body.rows(row_height, entry_count, |mut row| {
                let row_index = row.index();
                for field in fields {
                    row.col(|ui| {
                        edit_cell(ui, file, table_id, row_index, field, dirty, &mut error);
                    });
                }
            });
        });
    if let Some(message) = error {
        state.status = Some(crate::state::Status::error(message));
    }
}

fn table_lookup(
    loaded: &LoadedFile,
    table_id: TableId,
) -> Result<Option<(String, &'static [FieldDescriptor], usize)>> {
    Ok(loaded
        .file
        .tables()?
        .iter()
        .find(|table| table.id == table_id)
        .map(|table| (table.label.to_owned(), table.fields, table.entry_count)))
}

fn edit_cell(
    ui: &mut egui::Ui,
    file: &mut RLBFile,
    table_id: TableId,
    row: usize,
    field: &FieldDescriptor,
    dirty: &mut bool,
    error: &mut Option<String>,
) {
    let Some(value) = file.get_field(table_id, row, field.name) else {
        ui.weak("—");
        return;
    };

    let changed = match value {
        Value::Integer(mut v) => {
            let mut widget = egui::DragValue::new(&mut v)
                .speed(0)
                .hexadecimal(1, true, true)
                .prefix("0x");

            if let FieldConstraint::IntegerRange { min, max } = &field.constraint {
                widget = widget.range(*min..=*max);
            }

            let response = ui.add(widget);
            response.changed().then(|| Value::Integer(v))
        }
        Value::Boolean(mut b) => {
            let response = ui.checkbox(&mut b, "");
            response.changed().then(|| Value::Boolean(b))
        }
        Value::String(s) => {
            let mut text = s.unwrap_or_default();
            let response = ui.text_edit_singleline(&mut text);
            response
                .changed()
                .then(|| Value::String(if text.is_empty() { None } else { Some(text) }))
        }
    };

    if let Some(new_value) = changed {
        match file.set_field(table_id, row, field.name, new_value) {
            Ok(()) => *dirty = true,
            Err(e) => *error = Some(format!("could not update \"{}\": {e}", field.name)),
        }
    }
}

fn field_hint(field: &FieldDescriptor) -> String {
    let kind = match field.kind {
        FieldKind::Integer => "integer",
        FieldKind::String => "string",
        FieldKind::Boolean => "boolean",
    };

    match &field.constraint {
        FieldConstraint::None => kind.to_owned(),
        FieldConstraint::IntegerRange { min, max } => format!("{kind} ({min}..={max})"),
        FieldConstraint::TableIndex { table } => format!("{kind}, indexes into \"{table}\""),
    }
}
