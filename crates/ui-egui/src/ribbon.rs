//! A drafting ribbon for people moving from a modern tabbed CAD workspace.
//!
//! This is original CADCraft UI, using CADCraft's own artwork and commands. It
//! never reads or embeds another application's resource files.

use cadcraft_color::Color;
use egui::{Align2, FontId, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

type Tool = (Icon, &'static str, &'static str);

// Reserve an actual footer row for group captions. Previously the command
// buttons occupied the whole ribbon and the captions were clipped away.
const GROUP_BODY_HEIGHT: f32 = 82.0;
const GROUP_CAPTION_HEIGHT: f32 = 18.0;

struct Group {
    label: &'static str,
    tools: &'static [Tool],
}

// Keys are stable in saved preferences. The names on screen follow the
// familiar drafting tabs; both Portuguese and English are supported.
const TABS: &[(&str, &str, &str)] = &[
    ("home", "Início", "Home"),
    ("insert", "Inserir", "Insert"),
    ("annotate", "Anotar", "Annotate"),
    ("parametric", "Paramétrico", "Parametric"),
    ("view", "Vista", "View"),
    ("manage", "Gerenciar", "Manage"),
    ("output", "Saída", "Output"),
    ("3d", "3D", "3D"),
];

const HOME: &[Group] = &[
    Group {
        label: "Draw",
        tools: &[
            (Icon::Line, "line", "Line"),
            (Icon::Polyline, "pline", "Polyline"),
            (Icon::Circle, "circle", "Circle"),
            (Icon::Arc, "arc", "Arc"),
            (Icon::Rectangle, "rectang", "Rectangle"),
            (Icon::Hatch, "hatch", "Hatch"),
            (Icon::Ellipse, "ellipse", "Ellipse"),
            (Icon::Polygon, "polygon", "Polygon"),
        ],
    },
    Group {
        label: "Modify",
        tools: &[
            (Icon::Move, "move", "Move"),
            (Icon::Copy, "copy", "Copy"),
            (Icon::Rotate, "rotate", "Rotate"),
            (Icon::Mirror, "mirror", "Mirror"),
            (Icon::Trim, "trim", "Trim"),
            (Icon::Offset, "offset", "Offset"),
            (Icon::Fillet, "fillet", "Fillet"),
            (Icon::Erase, "erase", "Erase"),
        ],
    },
    Group {
        label: "Annotation",
        tools: &[
            (Icon::MText, "mtext", "Multiline Text"),
            (Icon::DimLinear, "dimlinear", "Linear"),
            (Icon::DimAligned, "dimaligned", "Aligned"),
            (Icon::MLeader, "mleader", "Multileader"),
        ],
    },
    Group {
        label: "Layers",
        tools: &[
            (Icon::LayerProps, "ui.dialog.layers", "Layer Properties Manager"),
            (Icon::LayerOff, "layoff", "Layer Off"),
            (Icon::LayerIso, "layiso", "Isolate Layer"),
            (Icon::MakeCurrent, "laymcur", "Make Current"),
            (Icon::LayerMatch, "laymch", "Match Layer"),
        ],
    },
    Group {
        label: "Block",
        tools: &[(Icon::Insert, "insert", "Insert Block"), (Icon::BlockCreate, "block", "Create Block"), (Icon::BlockEdit, "bedit", "Block Editor")],
    },
    Group { label: "Properties", tools: &[] },
    Group {
        label: "Groups",
        tools: &[(Icon::BlockCreate, "group", "Group"), (Icon::Explode, "ungroup", "Ungroup"), (Icon::BlockEdit, "groupedit", "Group Edit")],
    },
    Group { label: "Utilities", tools: &[(Icon::Measure, "dist", "Measure Distance"), (Icon::Area, "area", "Area"), (Icon::List, "list", "List")] },
    Group { label: "Clipboard", tools: &[(Icon::Copy, "cutclip", "Cut"), (Icon::Copy, "copyclip", "Copy"), (Icon::Insert, "pasteclip", "Paste")] },
];

const INSERT: &[Group] = &[
    Group {
        label: "Block",
        tools: &[(Icon::Insert, "insert", "Insert Block"), (Icon::BlockCreate, "block", "Create Block"), (Icon::BlockEdit, "bedit", "Block Editor")],
    },
    Group {
        label: "Attributes",
        tools: &[
            (Icon::AttDef, "attdef", "Define Attributes"),
            (Icon::BlockEdit, "attedit", "Edit Attributes..."),
            (Icon::Blocks, "battman", "Block Attribute Manager..."),
            (Icon::AttDef, "attsync", "Synchronize Attributes"),
        ],
    },
    Group {
        label: "Reference",
        tools: &[(Icon::Attach, "xref", "Reference Manager"), (Icon::Attach, "xattach", "DWG Reference..."), (Icon::Import, "import", "Import")],
    },
    Group {
        label: "Data",
        tools: &[
            (Icon::Table, "table", "Table"),
            (Icon::Table, "tablestyle", "Table Style..."),
            (Icon::Import, "dataextraction", "Data Extraction..."),
        ],
    },
];

const ANNOTATE: &[Group] = &[
    Group {
        label: "Text",
        tools: &[
            (Icon::Text, "text", "Single Line Text"),
            (Icon::MText, "mtext", "Multiline Text"),
            (Icon::TextStyle, "style", "Text Style"),
            (Icon::MText, "txt2mtxt", "Convert Text to Mtext"),
        ],
    },
    Group {
        label: "Dimension",
        tools: &[
            (Icon::DimLinear, "dimlinear", "Linear"),
            (Icon::DimAligned, "dimaligned", "Aligned"),
            (Icon::DimAngular, "dimangular", "Angular"),
            (Icon::DimRadius, "dimradius", "Radius"),
            (Icon::DimDiameter, "dimdiameter", "Diameter"),
            (Icon::DimBaseline, "dimbaseline", "Baseline"),
            (Icon::DimContinue, "dimcontinue", "Continue"),
            (Icon::DimStyle, "dimstyle", "Dimension Style"),
        ],
    },
    Group {
        label: "Leader",
        tools: &[
            (Icon::MLeader, "mleader", "Multileader"),
            (Icon::AddLeader, "mleaderedit.add", "Add Leader"),
            (Icon::RemoveLeader, "mleaderedit.remove", "Remove Leader"),
            (Icon::AlignLeaders, "mleaderalign", "Align Multileaders"),
        ],
    },
    Group { label: "Table", tools: &[(Icon::Table, "table", "Table"), (Icon::Table, "tablestyle", "Table Style...")] },
];

const PARAMETRIC: &[Group] = &[
    Group {
        label: "Geometric Constraints",
        tools: &[
            (Icon::Constraint, "autoconstrain", "Auto Constrain"),
            (Icon::Coincident, "gccoincident", "Coincident"),
            (Icon::Parallel, "gcparallel", "Parallel"),
            (Icon::Perpendicular, "gcperpendicular", "Perpendicular"),
            (Icon::Horizontal, "gchorizontal", "Horizontal"),
            (Icon::Vertical, "gcvertical", "Vertical"),
            (Icon::Tangent, "gctangent", "Tangent"),
            (Icon::Concentric, "gcconcentric", "Concentric"),
        ],
    },
    Group {
        label: "Dimensional Constraints",
        tools: &[
            (Icon::DimLinear, "dclinear", "Linear"),
            (Icon::DimAligned, "dcaligned", "Aligned"),
            (Icon::DimAngular, "dcangular", "Angular"),
            (Icon::DimRadius, "dcradius", "Radius"),
            (Icon::DimDiameter, "dcdiameter", "Diameter"),
            (Icon::Horizontal, "dchorizontal", "Horizontal"),
            (Icon::Vertical, "dcvertical", "Vertical"),
        ],
    },
    Group {
        label: "Manage",
        tools: &[
            (Icon::Parameters, "parameters", "Parameters Manager"),
            (Icon::DelConstraint, "delconstraint", "Delete Constraints"),
            (Icon::Gear, "constraintsettings", "Constraint Settings"),
            (Icon::Constraint, "constraintinfer", "Infer Constraints"),
        ],
    },
    Group {
        label: "Display",
        tools: &[
            (Icon::Constraint, "constraintbar.showall", "Show All Constraint Bars"),
            (Icon::DelConstraint, "constraintbar.hideall", "Hide All Constraint Bars"),
            (Icon::Annotation, "dcdisplay.showall", "Show All Dynamic Dimensions"),
            (Icon::DimLinear, "dcdisplay.hideall", "Hide All Dynamic Dimensions"),
        ],
    },
];

const VIEW: &[Group] = &[
    Group {
        label: "Navigate",
        tools: &[
            (Icon::ZoomExtents, "zoom.extents", "Zoom Extents"),
            (Icon::ZoomWindow, "zoom.window", "Zoom Window"),
            (Icon::Pan, "pan", "Pan"),
            (Icon::ZoomOut, "zoom.previous", "Zoom Previous"),
        ],
    },
    Group {
        label: "Views",
        tools: &[(Icon::Workspace, "view", "Named Views..."), (Icon::ZoomIn, "zoom.in", "Zoom In"), (Icon::ZoomOut, "zoom.out", "Zoom Out")],
    },
    Group { label: "Viewports", tools: &[(Icon::Workspace, "vports", "New Viewports..."), (Icon::Grid, "mview", "Viewports")] },
    Group {
        label: "Display",
        tools: &[
            (Icon::Workspace, "ui.toggle.viewcube", "ViewCube"),
            (Icon::Ortho, "ui.toggle.ucsicon", "UCS Icon"),
            (Icon::ZoomFit, "regen", "Regen"),
            (Icon::ZoomExtents, "regenall", "Regen All"),
        ],
    },
    Group {
        label: "Palettes",
        tools: &[
            (Icon::Layers, "ui.panel.layers", "Layers Palette"),
            (Icon::Properties, "ui.panel.properties", "Properties Inspector"),
            (Icon::Blocks, "ui.dialog.blocks", "Blocks"),
        ],
    },
];

const MANAGE: &[Group] = &[
    Group { label: "Utilities", tools: &[(Icon::Measure, "dist", "Measure Distance"), (Icon::Area, "area", "Area"), (Icon::List, "list", "List")] },
    Group {
        label: "Drawing",
        tools: &[
            (Icon::Blocks, "audit", "Audit"),
            (Icon::Erase, "purge", "Purge"),
            (Icon::Open, "recover", "Recover..."),
            (Icon::Measure, "units", "Units..."),
        ],
    },
    Group {
        label: "Settings",
        tools: &[
            (Icon::Gear, "options", "Options..."),
            (Icon::Osnap, "dsettings", "Drafting Settings..."),
            (Icon::Workspace, "ui.dock.reset", "Reset Panel Layout"),
            (Icon::Annotation, "ui.dialog.language", "Interface Language"),
        ],
    },
    Group {
        label: "Content",
        tools: &[(Icon::Layers, "ui.dialog.layers", "Layer Properties Manager"), (Icon::Blocks, "ui.dialog.blocks", "Blocks")],
    },
];

const OUTPUT: &[Group] = &[
    Group {
        label: "Plot",
        tools: &[
            (Icon::Plot, "plot", "Print"),
            (Icon::PageSetup, "pagesetup", "Page Setup Manager"),
            (Icon::Preview, "preview", "Plot Preview"),
            (Icon::Publish, "publish", "Batch Publish"),
        ],
    },
    Group { label: "Export", tools: &[(Icon::Export, "export", "Export"), (Icon::Plot, "exportpdf", "Export to PDF...")] },
    Group { label: "Styles", tools: &[(Icon::PageSetup, "stylesmanager", "Plot Style Manager..."), (Icon::PageSetup, "plotstyle", "Plot Style...")] },
];

const THREE_D: &[Group] = &[
    Group {
        label: "Navigate",
        tools: &[
            (Icon::Pan, "pan", "Pan"),
            (Icon::ZoomExtents, "zoom.extents", "Zoom Extents"),
            (Icon::ZoomWindow, "zoom.window", "Zoom Window"),
            (Icon::ZoomOut, "zoom.previous", "Zoom Previous"),
        ],
    },
    Group {
        label: "View",
        tools: &[
            (Icon::Workspace, "ui.toggle.viewcube", "ViewCube"),
            (Icon::Ortho, "ui.toggle.ucsicon", "UCS Icon"),
            (Icon::Workspace, "view", "Named Views..."),
        ],
    },
];

fn groups(tab: &str) -> &'static [Group] {
    match tab {
        "insert" => INSERT,
        "annotate" => ANNOTATE,
        "parametric" => PARAMETRIC,
        "view" => VIEW,
        "manage" => MANAGE,
        "output" => OUTPUT,
        "3d" => THREE_D,
        _ => HOME,
    }
}

/// Fit the actual localized label in the button, preserving whole words when a
/// second line is available. Ellipses are a last resort, never a fixed-length
/// shortcut: "Concordância" should not be shortened just for being long.
fn fitted_button_label(name: &str, allow_two_lines: bool, fits: impl Fn(&str) -> bool) -> String {
    if fits(name) {
        return name.to_owned();
    }
    if allow_two_lines {
        let mut best: Option<(usize, String)> = None;
        for (split_at, _) in name.match_indices(' ') {
            let (first, second) = name.split_at(split_at);
            let second = second.trim_start();
            if !first.is_empty() && !second.is_empty() && fits(first) && fits(second) {
                let score = first.chars().count().abs_diff(second.chars().count());
                // When both arrangements are equally balanced, prefer the
                // later split so phrases like "Linha de chamada" stay together.
                if !matches!(&best, Some((old_score, _)) if score > *old_score) {
                    best = Some((score, format!("{first}\n{second}")));
                }
            }
        }
        if let Some((_, lines)) = best {
            return lines;
        }
    }
    let mut shortened = name.to_owned();
    while !shortened.is_empty() && !fits(&format!("{shortened}…")) {
        shortened.pop();
    }
    format!("{shortened}…")
}

/// The large primary action and compact secondary actions share command and
/// accessibility handling. Only their icon/text geometry changes.
fn tool_button(ui: &mut egui::Ui, (icon, command, label): Tool, prominent: bool, size: egui::Vec2) -> bool {
    let t = Tokens::get();
    let known = cadcraft_engine::find_command(command).is_some() || crate::menus::is_ui_command(command);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if response.hovered() && known {
        ui.painter().rect_filled(rect, 3.0, t.control_hover);
    }
    let tint = if known { t.text } else { t.text_dim.gamma_multiply(0.55) };
    let icon_size = if prominent { 32.0 } else { 17.0 };
    let center = if prominent { pos2(rect.center().x, rect.top() + 29.0) } else { pos2(rect.left() + 11.0, rect.center().y) };
    icons::paint(ui.painter(), Rect::from_center_size(center, vec2(icon_size, icon_size)), icon, !known);
    let name = crate::i18n::t(label);
    // Main actions have short ribbon captions, while their tooltips and
    // accessible names retain the complete command description.
    let visible_name = if prominent {
        match label {
            "Layer Properties Manager" => crate::i18n::t("Layer Properties"),
            "Layers Palette" => crate::i18n::t("Layers"),
            "Multileader" => crate::i18n::t("Leader"),
            "Parameters Manager" => crate::i18n::t("Parameters"),
            "Plot Style Manager..." => crate::i18n::t("Styles"),
            "Properties Inspector" => crate::i18n::t("Properties"),
            "Reference Manager" => crate::i18n::t("Reference"),
            "Single Line Text" => crate::i18n::t("Text"),
            _ => name,
        }
    } else {
        name
    };
    let font_size = if prominent { 11.5 } else { 11.0 };
    let font = FontId::proportional(font_size);
    let text_width = if prominent { size.x - 9.0 } else { size.x - 29.0 };
    let display = fitted_button_label(visible_name, prominent || size.y >= 25.0, |line| {
        ui.painter().layout_no_wrap(line.to_owned(), font.clone(), tint).size().x <= text_width
    });
    let (text_pos, alignment) = if prominent {
        (pos2(rect.center().x, rect.bottom() - 17.0), Align2::CENTER_CENTER)
    } else {
        (pos2(rect.left() + 23.0, rect.center().y), Align2::LEFT_CENTER)
    };
    ui.painter().text(text_pos, alignment, display, font, tint);
    let response = response.on_hover_text(name);
    icons::describe_control(ui, &response, name, None, false);
    known && response.clicked()
}

/// AutoCAD-like command groups: one prominent action and a tight stack of
/// secondary actions, while leaving the full command names in accessibility.
fn command_group(ui: &mut egui::Ui, tab: &str, group: &Group, roomy: bool, clicked: &mut Option<&'static str>) {
    let Some(&primary) = group.tools.first() else { return };
    let secondary = &group.tools[1..];
    let dense = secondary.len() > 3;
    let columns = if dense { 2 } else { 1 };
    // Give localized labels room on desktop screens; on narrow windows keep
    // the earlier compact geometry so the properties controls stay reachable.
    let secondary_size = match (roomy, dense) {
        (true, true) => vec2(118.0, 20.0),
        (true, false) => vec2(154.0, 27.0),
        (false, true) => vec2(92.0, 20.0),
        (false, false) => vec2(116.0, 26.5),
    };
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        if tool_button(ui, primary, true, vec2(68.0, 82.0)) {
            *clicked = Some(primary.1);
        }
        if !secondary.is_empty() {
            egui::Grid::new(("cc_ribbon_secondary", tab, group.label)).num_columns(columns).spacing(vec2(1.0, 1.0)).show(ui, |ui| {
                for (index, &tool) in secondary.iter().enumerate() {
                    if tool_button(ui, tool, false, secondary_size) {
                        *clicked = Some(tool.1);
                    }
                    if (index + 1) % columns == 0 {
                        ui.end_row();
                    }
                }
            });
        }
    });
}

/// Current layer picker in the Home ribbon, with the same per-layer controls as
/// the optional Layers palette. The document, not UI state, is authoritative.
fn layer_controls(app: &mut CadApp, ui: &mut egui::Ui, clicked: &mut Option<&'static str>) {
    let selected = app.session.selection();
    let selected_ids: Vec<String> = selected.iter().map(|handle| handle.hex()).collect();
    let Ok(doc) = app.session.doc() else { return };
    let active = doc.header.str("CLAYER", "0");
    let selected_layers: Vec<_> = selected.iter().filter_map(|handle| doc.entity(*handle)).map(|entity| entity.common.layer.clone()).collect();
    let current = selected_layers.first().cloned().unwrap_or_else(|| active.clone());
    let current_label = if selected_layers.iter().all(|name| name.eq_ignore_ascii_case(&current)) { current.clone() } else { "*VARIES*".into() };
    let layers: Vec<_> = doc.layers.iter().map(|l| (l.name.clone(), l.on, l.frozen, l.locked, l.color)).collect();
    let current_color = layers.iter().find(|layer| layer.0.eq_ignore_ascii_case(&current)).map_or(Color::Index(7), |layer| layer.4);
    let mut next_current = None;
    let mut next_state: Option<(String, &'static str, bool)> = None;
    let mut match_clicked = false;

    ui.set_min_width(352.0);
    ui.horizontal(|ui| {
        if tool_button(ui, (Icon::LayerProps, "ui.dialog.layers", "Layer Properties"), true, vec2(104.0, GROUP_BODY_HEIGHT)) {
            *clicked = Some("ui.dialog.layers");
        }
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                let rgb = current_color.resolve(Color::Index(7), Color::Index(7));
                let (swatch, _) = ui.allocate_exact_size(vec2(19.0, 19.0), Sense::hover());
                ui.painter().rect_filled(swatch, 1.0, egui::Color32::from_rgb(rgb.0, rgb.1, rgb.2));
                ui.painter().rect_stroke(swatch, 1.0, Stroke::new(1.0, Tokens::get().border), egui::StrokeKind::Inside);
                craft_ui::ComboBox::from_id_salt("ribbon-current-layer")
                    .width(214.0)
                    .selected_text(&current_label)
                    .show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(310.0).show(ui, |ui| {
                            for (name, on, frozen, locked, color) in &layers {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 3.0;
                                    if icons::button(ui, if *on { Icon::Bulb } else { Icon::BulbOff }, 20.0, crate::i18n::t("On/Off"), false)
                                        .clicked()
                                    {
                                        next_state = Some((name.clone(), "on", !*on));
                                    }
                                    if icons::button(
                                        ui,
                                        if *frozen { Icon::Snowflake } else { Icon::Sun },
                                        20.0,
                                        crate::i18n::t("Freeze/Thaw"),
                                        false,
                                    )
                                    .clicked()
                                    {
                                        next_state = Some((name.clone(), "frozen", !*frozen));
                                    }
                                    if icons::button(ui, if *locked { Icon::Lock } else { Icon::Unlock }, 20.0, crate::i18n::t("Lock/Unlock"), false)
                                        .clicked()
                                    {
                                        next_state = Some((name.clone(), "locked", !*locked));
                                    }
                                    let rgb = color.resolve(Color::Index(7), Color::Index(7));
                                    ui.colored_label(egui::Color32::from_rgb(rgb.0, rgb.1, rgb.2), "■");
                                    if ui.selectable_label(name.eq_ignore_ascii_case(&current), name).clicked() {
                                        next_current = Some(name.clone());
                                        ui.close();
                                    }
                                });
                            }
                        });
                    })
                    .response
                    .accessible_name(crate::i18n::t("Current layer"));
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                for (icon, command, label) in [
                    (Icon::LayerOff, "layoff", "Layer Off"),
                    (Icon::LayerIso, "layiso", "Isolate Layer"),
                    (Icon::LayerFreeze, "layfrz", "Freeze Layer"),
                ] {
                    if icons::button(ui, icon, 23.0, crate::i18n::t(label), false).clicked() {
                        *clicked = Some(command);
                    }
                }
                if ui.small_button(crate::i18n::t("Make Current")).clicked() {
                    *clicked = Some("laymcur");
                }
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                let icon_clicked = icons::button(ui, Icon::LayerMatch, 23.0, crate::i18n::t("Match Layer"), false).clicked();
                let label_clicked = ui.small_button(crate::i18n::t("Match Layer")).clicked();
                if icon_clicked || label_clicked {
                    match_clicked = true;
                }
            });
        });
    });
    if match_clicked {
        if selected_ids.is_empty() {
            app.set_status(crate::i18n::t("Select objects to match to the current layer"));
        } else if let Err(error) = app.run("laymch", json!({ "handles": selected_ids.clone(), "layer": active })) {
            app.set_status(error);
        }
    }
    if let Some(name) = next_current {
        let (command, params) = if selected_ids.is_empty() {
            ("layer.current", json!({ "name": name }))
        } else {
            ("properties.set", json!({ "handles": selected_ids, "layer": name }))
        };
        if let Err(error) = app.run(command, params) {
            app.set_status(error);
        }
    }
    if let Some((name, key, value)) = next_state
        && let Err(error) = app.run("layer.set", json!({ "name": name, key: value }))
    {
        app.set_status(error);
    }
}

/// A real linetype/lineweight sample alongside the property selectors. Positive
/// pattern elements are drawn, negative elements are gaps, and zeros are dots.
/// This uses the document's own linetype pattern rather than a decorative icon.
fn line_sample(ui: &mut egui::Ui, pattern: &[f64], stroke_width: f32, width: f32) {
    let (rect, _) = ui.allocate_exact_size(vec2(width, 21.0), Sense::hover());
    let t = Tokens::get();
    ui.painter().rect_filled(rect, 2.0, t.field);
    ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    let left = rect.left() + 3.0;
    let right = rect.right() - 3.0;
    let y = rect.center().y;
    let stroke = Stroke::new(stroke_width, t.text);
    let period: f32 = pattern.iter().filter(|part| part.is_finite()).map(|part| part.abs() as f32).sum();
    if period <= f32::EPSILON {
        ui.painter().line_segment([pos2(left, y), pos2(right, y)], stroke);
        return;
    }
    let scale = ((right - left) / (period * 2.0)).clamp(0.5, 20.0);
    let mut x = left;
    for _ in 0..8 {
        for part in pattern.iter().copied().filter(|part| part.is_finite()) {
            if x >= right {
                return;
            }
            let length = if part.abs() < f64::EPSILON { 2.5 } else { (part.abs() as f32 * scale).max(1.5) };
            let end = (x + length).min(right);
            if part > 0.0 {
                ui.painter().line_segment([pos2(x, y), pos2(end, y)], stroke);
            } else if part.abs() < f64::EPSILON {
                ui.painter().circle_filled(pos2(x, y), stroke_width.max(1.2), t.text);
            }
            x += length;
        }
    }
}

fn weight_sample_width(label: &str) -> f32 {
    label
        .strip_suffix(" mm")
        .and_then(|millimetres| millimetres.parse::<f32>().ok())
        .map_or(1.6, |millimetres| (1.2 + millimetres * 2.3).clamp(1.3, 4.2))
}

/// AutoCAD-style three-row property controls. With no selection these edit the
/// drawing's current defaults; with a selection they edit its objects.
fn property_controls(app: &mut CadApp, ui: &mut egui::Ui) {
    let selected = app.session.selection();
    let ids: Vec<String> = selected.iter().map(|handle| handle.hex()).collect();
    let Ok(doc) = app.session.doc() else { return };
    let properties: Vec<_> = selected
        .iter()
        .filter_map(|handle| doc.entity(*handle))
        .map(|entity| (entity.common.color, entity.common.linetype.clone(), entity.common.lineweight.name(), entity.common.layer.clone()))
        .collect();
    let header_color = doc
        .header
        .get("CECOLOR_RGB")
        .and_then(|value| value.as_i64())
        .and_then(|value| u32::try_from(value).ok())
        .map_or_else(|| Color::from_aci(doc.header.i64("CECOLOR", 256) as i16), |rgb| Color::True(cadcraft_color::Rgb::from_u32(rgb)));
    let color = properties.first().map_or(header_color, |property| property.0);
    let color_label = if properties.iter().all(|property| property.0 == color) { color.name() } else { "*VARIES*".into() };
    let layer_name = properties.first().map_or_else(|| doc.header.str("CLAYER", "0"), |property| property.3.clone());
    let current_layer = doc.layers.iter().find(|layer| layer.name.eq_ignore_ascii_case(&layer_name));
    let layer_color = current_layer.map_or(Color::WHITE, |layer| layer.color);
    let resolved_color = color.resolve(layer_color, Color::WHITE);
    let linetype = properties.first().map_or_else(|| doc.header.str("CELTYPE", "ByLayer"), |property| property.1.clone());
    let linetype_label = if properties.iter().all(|property| property.1 == linetype) { linetype.clone() } else { "*VARIES*".into() };
    let lineweight = properties
        .first()
        .map_or_else(|| cadcraft_doc::Lineweight::from_dxf(doc.header.i64("CELWEIGHT", -1) as i16).name(), |property| property.2.clone());
    let weight_label = if properties.iter().all(|property| property.2 == lineweight) { lineweight } else { "*VARIES*".into() };
    let types: Vec<(String, Vec<f64>)> =
        doc.linetypes.iter().map(|kind| (kind.name.clone(), kind.pattern.iter().map(|dash| dash.length).collect())).collect();
    let layer_linetype = current_layer.map_or("Continuous", |layer| layer.linetype.as_str());
    let layer_weight = current_layer.map_or(cadcraft_doc::Lineweight::Default, |layer| layer.lineweight).name();
    let effective_linetype = if linetype == "ByLayer" { layer_linetype } else { linetype.as_str() };
    let by_layer_pattern =
        types.iter().find(|(name, _)| name.eq_ignore_ascii_case(layer_linetype)).map_or(&[][..], |(_, pattern)| pattern.as_slice());
    let selected_pattern =
        types.iter().find(|(name, _)| name.eq_ignore_ascii_case(effective_linetype)).map_or(&[][..], |(_, pattern)| pattern.as_slice());
    let effective_weight = if weight_label == "ByLayer" { &layer_weight } else { &weight_label };
    let mut edit: Option<(&'static str, serde_json::Value)> = None;

    ui.set_min_width(229.0);
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 2.0;
        egui::Frame::NONE.fill(Tokens::get().field).stroke(Stroke::new(1.0, Tokens::get().border)).show(ui, |ui| {
            let color_popup_id = ui.make_persistent_id("ribbon-object-color");
            let color_control_origin = ui.cursor().min;
            if let Some(picked) = crate::layers::color_button_resolved(ui, color_popup_id, color, resolved_color, &color_label, true, 219.0) {
                edit = Some(("color", json!(picked.name())));
            }
            ui.painter().text(
                pos2(color_control_origin.x + 209.0, color_control_origin.y + 10.0),
                Align2::CENTER_CENTER,
                "▾",
                FontId::proportional(11.0),
                Tokens::get().text_dim,
            );
        });
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 3.0;
            line_sample(ui, selected_pattern, 1.85, 44.0);
            craft_ui::ComboBox::from_id_salt("ribbon-object-linetype")
                .width(181.0)
                .selected_text(crate::i18n::t(&linetype_label))
                .show_ui(ui, |ui| {
                    for choice in std::iter::once("ByLayer".to_string())
                        .chain(std::iter::once("ByBlock".to_string()))
                        .chain(types.iter().filter(|(name, _)| !["ByLayer", "ByBlock"].contains(&name.as_str())).map(|(name, _)| name.clone()))
                    {
                        let pattern = if choice == "ByLayer" {
                            by_layer_pattern
                        } else {
                            types.iter().find(|(name, _)| name == &choice).map_or(&[][..], |(_, pattern)| pattern.as_slice())
                        };
                        ui.horizontal(|ui| {
                            line_sample(ui, pattern, 1.85, 48.0);
                            if ui.selectable_label(choice == linetype_label, crate::i18n::t(&choice)).clicked() {
                                edit = Some(("linetype", json!(choice)));
                                ui.close();
                            }
                        });
                    }
                })
                .response
                .accessible_name(crate::i18n::t("Linetype"));
        });
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 3.0;
            line_sample(ui, &[], weight_sample_width(effective_weight), 44.0);
            craft_ui::ComboBox::from_id_salt("ribbon-object-lineweight")
                .width(181.0)
                .selected_text(crate::i18n::t(&weight_label))
                .show_ui(ui, |ui| {
                    egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
                        for choice in ["ByLayer", "ByBlock", "Default"] {
                            ui.horizontal(|ui| {
                                line_sample(ui, &[], if choice == "ByLayer" { weight_sample_width(&layer_weight) } else { 1.6 }, 48.0);
                                if ui.selectable_label(weight_label == choice, crate::i18n::t(choice)).clicked() {
                                    edit = Some(("lineweight", json!(choice)));
                                    ui.close();
                                }
                            });
                        }
                        for standard in cadcraft_doc::Lineweight::STANDARD {
                            let label = format!("{:.2} mm", f64::from(standard) / 100.0);
                            ui.horizontal(|ui| {
                                line_sample(ui, &[], weight_sample_width(&label), 48.0);
                                if ui.selectable_label(weight_label == label, &label).clicked() {
                                    edit = Some(("lineweight", json!(f64::from(standard) / 100.0)));
                                    ui.close();
                                }
                            });
                        }
                    });
                })
                .response
                .accessible_name(crate::i18n::t("Lineweight"));
        });
    });

    if let Some((field, value)) = edit {
        let (command, params) = if ids.is_empty() {
            match field {
                "color" => ("color", json!({ "color": value })),
                "linetype" => ("linetype", json!({ "current": value })),
                _ => ("lweight", json!({ "lineweight": value })),
            }
        } else {
            ("properties.set", json!({ "handles": ids, field: value }))
        };
        if let Err(error) = app.run(command, params) {
            app.set_status(error);
        }
    }
}

/// Draw the original CADCraft ribbon immediately beneath the menu bar.
pub fn show(app: &mut CadApp, ui: &mut egui::Ui) {
    if !app.ui.autocad_ribbon {
        return;
    }
    let t = Tokens::get();
    let mut selected = app.ui.ribbon_tab.clone();
    if !TABS.iter().any(|(key, _, _)| *key == selected) {
        selected = "home".into();
    }
    let portuguese = app.language().starts_with("pt");
    let mut clicked = None;
    egui::Panel::top("cc_drafting_ribbon")
        .exact_size(158.0)
        .frame(egui::Frame::NONE.fill(t.chrome).inner_margin(egui::Margin::symmetric(7, 3)))
        .show(ui, |ui| {
            let roomy = ui.available_width() >= 1600.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                for (key, pt, en) in TABS {
                    let title = if portuguese { *pt } else { *en };
                    if ui.selectable_label(selected == *key, title).clicked() {
                        selected = (*key).into();
                    }
                }
            });
            ui.separator();
            egui::ScrollArea::horizontal().id_salt("cc_ribbon_groups").auto_shrink([false, false]).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;
                    for group in groups(&selected) {
                        ui.vertical(|ui| {
                            ui.set_min_height(GROUP_BODY_HEIGHT + GROUP_CAPTION_HEIGHT);
                            let body_top = ui.cursor().top();
                            if selected == "home" && group.label == "Layers" {
                                layer_controls(app, ui, &mut clicked);
                            } else if selected == "home" && group.label == "Properties" {
                                property_controls(app, ui);
                            } else {
                                command_group(ui, &selected, group, roomy, &mut clicked);
                            }
                            let body_used = ui.cursor().top() - body_top;
                            ui.add_space((GROUP_BODY_HEIGHT - body_used).max(0.0));
                            ui.allocate_ui_with_layout(
                                vec2(ui.min_rect().width(), GROUP_CAPTION_HEIGHT),
                                egui::Layout::top_down(egui::Align::Center),
                                |ui| {
                                    ui.label(egui::RichText::new(crate::i18n::t(group.label)).size(11.0).color(t.text_dim));
                                },
                            );
                        });
                        ui.separator();
                    }
                });
            });
        });
    app.ui.ribbon_tab = selected;
    if let Some(command) = clicked {
        app.start(command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drafting_workspace_uses_top_ribbon_and_restores_classic_palettes_on_request() {
        use crate::Services;
        use crate::docking::Panel;
        use cadcraft_engine::Session;

        let mut app = CadApp::new(Session::default(), Services::default());
        crate::docking::validate(&app.ui.docking).unwrap();
        assert!(!app.ui.docking.layout.contains(&Panel::Properties));
        assert!(!app.ui.docking.layout.contains(&Panel::Layers));
        assert!(!app.ui.docking.layout.contains(&Panel::ToolSets));
        app.run("ui.dock", json!({ "operation": "open", "panel": "layers" })).unwrap();
        assert!(app.ui.docking.layout.contains(&Panel::Layers), "the old palette remains available on demand");
        app.run("ui.dock.reset", json!({})).unwrap();
        assert!(!app.ui.docking.layout.contains(&Panel::Layers), "ribbon layout reset hides the redundant palette");
        assert!(!app.ui.docking.layout.contains(&Panel::Properties), "properties live in the ribbon too");
        app.ui.autocad_ribbon = false;
        app.run("ui.dock.reset", json!({})).unwrap();
        assert!(app.ui.docking.layout.contains(&Panel::Layers), "classic layout still restores both palettes");
        assert!(app.ui.docking.layout.contains(&Panel::Properties));
    }

    #[test]
    fn switching_ribbon_modes_updates_side_palette_visibility() {
        use crate::{Services, docking::Panel};
        use cadcraft_engine::Session;

        let mut app = CadApp::new(Session::default(), Services::default());
        app.run("ui.toggle.autocad_ribbon", json!({ "on": false })).unwrap();
        assert!(app.ui.show_palettes);
        app.run("ui.dock.reset", json!({})).unwrap();
        assert!(app.ui.docking.layout.contains(&Panel::Layers));
        assert!(app.ui.docking.layout.contains(&Panel::Properties));
        app.run("ui.toggle.autocad_ribbon", json!({ "on": true })).unwrap();
        assert!(!app.ui.show_palettes);
        app.run("ui.dock.reset", json!({})).unwrap();
        assert!(!app.ui.docking.layout.contains(&Panel::Layers));
        assert!(!app.ui.docking.layout.contains(&Panel::Properties));
    }

    #[test]
    fn home_tab_places_layers_between_annotation_and_block_and_properties_after_block() {
        let groups: Vec<_> = HOME.iter().map(|group| group.label).collect();
        assert_eq!(groups, ["Draw", "Modify", "Annotation", "Layers", "Block", "Properties", "Groups", "Utilities", "Clipboard"]);
        for command in ["layer.current", "layer.set", "laymcur", "laymch", "color", "linetype", "lweight", "properties.set"] {
            assert!(cadcraft_engine::find_command(command).is_some(), "{command} must remain available to the ribbon");
        }
    }

    #[test]
    fn every_ribbon_tab_has_groups_and_all_keys_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for (key, _, _) in TABS {
            assert!(seen.insert(*key), "duplicate ribbon tab: {key}");
            assert!(!groups(key).is_empty(), "empty ribbon tab: {key}");
        }
    }

    #[test]
    fn ribbon_buttons_only_use_available_commands() {
        for (key, _, _) in TABS {
            for group in groups(key) {
                for (_, command, _) in group.tools {
                    assert!(
                        cadcraft_engine::find_command(command).is_some() || crate::menus::is_ui_command(command),
                        "unknown command on ribbon tab {key}: {command}"
                    );
                }
            }
        }
    }

    #[test]
    fn secondary_tabs_group_existing_commands_without_duplicates() {
        let expected: &[(&str, &[&str])] = &[
            ("insert", &["Block", "Attributes", "Reference", "Data"]),
            ("annotate", &["Text", "Dimension", "Leader", "Table"]),
            ("parametric", &["Geometric Constraints", "Dimensional Constraints", "Manage", "Display"]),
            ("view", &["Navigate", "Views", "Viewports", "Display", "Palettes"]),
            ("manage", &["Utilities", "Drawing", "Settings", "Content"]),
            ("output", &["Plot", "Export", "Styles"]),
            ("3d", &["Navigate", "View"]),
        ];
        for &(tab, names) in expected {
            let actual: Vec<_> = groups(tab).iter().map(|group| group.label).collect();
            assert_eq!(actual.as_slice(), names, "Ribbon groups on tab {tab}");
            let mut commands = std::collections::HashSet::new();
            for group in groups(tab) {
                for (_, command, _) in group.tools {
                    assert!(commands.insert(*command), "duplicated command on tab {tab}: {command}");
                }
            }
        }
        assert!(THREE_D.iter().flat_map(|group| group.tools).all(|(_, command, _)| *command != "3dorbit"));
    }

    #[test]
    fn lineweight_samples_reflect_the_selected_thickness() {
        assert!(weight_sample_width("0.05 mm") < weight_sample_width("0.50 mm"));
        assert!(weight_sample_width("0.50 mm") < weight_sample_width("2.11 mm"));
        assert_eq!(weight_sample_width("ByLayer"), 1.6);
        assert_eq!(weight_sample_width("2.11 mm"), 4.2);
    }

    #[test]
    fn button_labels_fit_by_available_width_and_wrap_before_truncating() {
        let at_most_16_chars = |line: &str| line.chars().count() <= 16;
        assert_eq!(fitted_button_label("Concordância", false, at_most_16_chars), "Concordância");
        assert_eq!(fitted_button_label("Linha de chamada múltipla", true, at_most_16_chars), "Linha de chamada\nmúltipla");
        assert_eq!(fitted_button_label("Propriedades da camada", true, at_most_16_chars), "Propriedades\nda camada");
        assert_eq!(fitted_button_label("Line", false, at_most_16_chars), "Line");
        assert_eq!(fitted_button_label("Concordância", false, |line| line.chars().count() <= 9), "Concordâ…");
    }
}
