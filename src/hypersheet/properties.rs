pub struct HyperSheetProperties {
  pub no_date_message: String,
  pub wheel_h_factor: f32,
  pub wheel_v_factor: f32,
  pub font: String,
  pub color: String,
  pub background_color: String,
  pub selection_font: String,
  pub selection_font_color: String,
  pub selection_background_color: String,
  pub column_header_font: String,
  pub column_header_color: String,
  pub column_header_foreground_selection_font: String,
  pub column_header_background_color: String,
  pub column_header_foreground_selection_color: String,
  pub column_header_background_selection_color: String,
  pub column_header_halign: String,
  pub column_header_renderer: String,
  pub column_header_format: String,
  pub row_header_font: String,
  pub row_header_color: String,
  pub row_header_background_color: String,
  pub row_header_foreground_selection_color: String,
  pub row_header_foreground_selection_font: String,
  pub row_header_background_selection_color: String,
  pub background_color2: String,
  pub tree_header_font: String,
  pub tree_header_color: String,
  pub tree_header_background_color: String,
  pub tree_header_foreground_selection_color: String,
  pub tree_header_foreground_selection_font: String,
  pub tree_header_background_selection_color: String,
  pub filter_font: String,
  pub filter_color: String,
  pub filter_background_color: String,
  pub filter_foreground_selection_color: String,
  pub filter_background_selection_color: String,
  pub filter_halign: String,
  pub filter_renderer: String,
  pub filter_editor: String,
  pub filterable: bool,
  pub show_filter_row: bool,
  pub voffset: u32,
  pub scrollbar_hover_over: String,
  pub scrollbar_hover_off: String,
  pub scrolling_enabled: bool,
  pub v_scrollbar_class_prefix: String,
  pub h_scrollbar_class_prefix: String,
  pub halign: String,
  pub cell_padding: u32,
  pub icon_padding: u32,
  pub render_falsy: bool,
  pub headerify: String,
  pub grid_lines_h: bool,
  pub grid_lines_hwidth: u32,
  pub grid_lines_hcolor: String,
  pub grid_lines_v: bool,
  pub grid_lines_vwidth: u32,
  pub grid_lines_vcolor: String,
  pub grid_lines_column_header: bool,
  pub grid_lines_row_header: bool,
  pub grid_lines_user_data_area: bool,
  pub grid_border: bool,
  pub grid_border_left: bool,
  pub grid_border_right: bool,
  pub grid_border_top: bool,
  pub grid_border_bottom: bool,
  pub fixed_lines_hwidth: u32,
  pub fixed_lines_hcolor: String,
  pub fixed_lines_vwidth: u32,
  pub fixed_lines_vcolor: String,
  pub box_sizing: String,
  pub default_row_height: u32,
  pub default_column_width: u32,
  pub minimum_column_width: u32,
  pub resize_column_in_place: bool,
  pub repaint_interval_rate: u32,
  pub repaint_immediately: bool,
  pub use_hi_dpi: bool,
  pub feedback_count: u32,
  pub feedback_effect: String,
  pub read_only: bool,
  pub fixed_column_count: u32,
  pub fixed_row_count: u32,
  pub row_header_numbers: bool,
  pub row_header_checkboxes: bool,
  pub show_tree_column: bool,
  pub tree_renderer: String,
  pub show_header_row: bool,
  pub cell_selection: bool,
  pub column_selection: bool,
  pub row_selection: bool,
  pub single_row_selection_mode: bool,
  pub selection_region_overlay_color: String,
  pub selection_region_outline_color: String,
  pub column_autosizing: bool,
  pub row_number_autosizing: bool,
  pub tree_column_autosizing: bool,
  pub column_autosizing_max: u32,
  pub tree_column_autosizing_max: u32,
  pub header_text_wrapping: bool,
  pub row_resize: bool,
  pub editable: bool,
  pub edit_on_double_click: bool,
  pub edit_on_keydown: bool,
  pub edit_on_next_cell: bool,
  pub unsortable: bool,
  pub sort_on_double_click: bool,
  pub max_sort_columns: u32,
  pub sort_on_hidden_columns: bool,
  pub checkbox_only_row_selections: bool,
  pub auto_select_rows: bool,
  pub auto_select_columns: bool,
  pub collapse_cell_selections: bool,
  pub renderer: String,
  pub grid_renderer: String,
  pub link: bool,
  pub link_target: String,
  pub link_on_hover: bool,
  pub link_color: String,
  pub link_visited_color: String,
  pub link_color_on_hover: bool,
  pub strike_through: bool,
  pub multiple_selections: bool,
  pub enable_continuous_repaint: bool,
  pub columns_reorderable: bool,
  pub column_grab_margin: u32,
  pub column_clip: bool,
  pub restore_row_selections: bool,
  pub restore_column_selections: bool,
  pub truncate_text_with_ellipsis: bool,
}

impl HyperSheetProperties {
  pub fn default() -> Self {
    Self {
      no_date_message: "No items to display".to_string(),
      wheel_h_factor: 0.01,
      wheel_v_factor: 0.01,
      font: "13px Tahoma, Geneva, sans-seriff".to_string(),
      color: "rgb(25, 25, 25)".to_string(),
      background_color: "rgb(241, 241, 241)".to_string(),
      selection_font: "bold 13px Tahoma, Geneva, sans-serif".to_string(),
      selection_font_color: "rgb(0, 0, 128)".to_string(),
      selection_background_color: "rgba(147, 185, 255, 0.625)".to_string(),
      column_header_font: "12px Tahoma, Geneva, sans-serif".to_string(),
      column_header_color: "rgb(25, 25, 25)".to_string(),
      column_header_foreground_selection_font: "bold 12px Tahoma, Geneva, sans-serif".to_string(),
      column_header_background_color: "rgb(223, 227, 232)".to_string(),
      column_header_foreground_selection_color: "rgb(80, 80, 80)".to_string(),
      column_header_background_selection_color: "rgba(255, 220, 97, 0.45)".to_string(),
      column_header_halign: "center".to_string(),
      column_header_renderer: "SimpleCell".to_string(),
      column_header_format: "header".to_string(),
      row_header_font: "12px Tahoma, Geneva, sans-serif".to_string(),
      row_header_color: "rgb(25, 25, 25)".to_string(),
      row_header_background_color: "rgb(223, 227, 232)".to_string(),
      row_header_foreground_selection_color: "rgb(80, 80, 80)".to_string(),
      row_header_foreground_selection_font: "bold 12px Tahoma, Geneva, sans-serif".to_string(),
      row_header_background_selection_color: "rgba(255, 220, 97, 0.45)".to_string(),
      background_color2: "rgb(201, 201, 201)".to_string(),
      tree_header_font: "12px Tahoma, Geneva, sans-serif".to_string(),
      tree_header_color: "rgb(25, 25, 25)".to_string(),
      tree_header_background_color: "rgb(223, 227, 232)".to_string(),
      tree_header_foreground_selection_color: "rgb(80, 80, 80)".to_string(),
      tree_header_foreground_selection_font: "bold 12px Tahoma, Geneva, sans-serif".to_string(),
      tree_header_background_selection_color: "rgba(255, 220, 97, 0.45)".to_string(),
      filter_font: "12px Tahoma, Geneva, sans-serif".to_string(),
      filter_color: "rgb(25, 25, 25)".to_string(),
      filter_background_color: "white".to_string(),
      filter_foreground_selection_color: "rgb(25, 25, 25)".to_string(),
      filter_background_selection_color: "rgb(255, 220, 97)".to_string(),
      filter_halign: "center".to_string(),
      filter_renderer: "SimpleCell".to_string(),
      filter_editor: "TextField".to_string(),
      filterable: true,
      show_filter_row: false,
      voffset: 0,
      scrollbar_hover_over: "visible".to_string(),
      scrollbar_hover_off: "hidden".to_string(),
      scrolling_enabled: true,
      v_scrollbar_class_prefix: "".to_string(),
      h_scrollbar_class_prefix: "".to_string(),
      halign: "center".to_string(),
      cell_padding: 5,
      icon_padding: 3,
      render_falsy: false,
      headerify: "toTitle".to_string(),
      grid_lines_h: true,
      grid_lines_hwidth: 1,
      grid_lines_hcolor: "rgb(199, 199, 199)".to_string(),
      grid_lines_v: true,
      grid_lines_vwidth: 1,
      grid_lines_vcolor: "rgb(199, 199, 199)".to_string(),
      grid_lines_column_header: true,
      grid_lines_row_header: true,
      grid_lines_user_data_area: true,
      grid_border: false,
      grid_border_left: false,
      grid_border_right: false,
      grid_border_top: false,
      grid_border_bottom: false,
      fixed_lines_hwidth: 2,
      fixed_lines_hcolor: "rgb(164,164,164)".to_string(),
      fixed_lines_vwidth: 2,
      fixed_lines_vcolor: "rgb(164,164,164)".to_string(),
      box_sizing: "content-box".to_string(),
      default_row_height: 14,
      default_column_width: 100,
      minimum_column_width: 5,
      resize_column_in_place: false,
      repaint_interval_rate: 60,
      repaint_immediately: false,
      use_hi_dpi: true,
      feedback_count: 3,
      feedback_effect: "shaker".to_string(),
      read_only: false,
      fixed_column_count: 0,
      fixed_row_count: 0,
      row_header_numbers: true,
      row_header_checkboxes: true,
      show_tree_column: true,
      tree_renderer: "SimpleCell".to_string(),
      show_header_row: true,
      cell_selection: true,
      column_selection: true,
      row_selection: true,
      single_row_selection_mode: true,
      selection_region_overlay_color: "transparent".to_string(),
      selection_region_outline_color: "rgb(69, 69, 69)".to_string(),
      column_autosizing: true,
      row_number_autosizing: true,
      tree_column_autosizing: true,
      column_autosizing_max: 400,
      tree_column_autosizing_max: 400,
      header_text_wrapping: false,
      row_resize: false,
      editable: true,
      edit_on_double_click: true,
      edit_on_keydown: true,
      edit_on_next_cell: false,
      unsortable: false,
      sort_on_double_click: true,
      max_sort_columns: 3,
      sort_on_hidden_columns: true,
      checkbox_only_row_selections: false,
      auto_select_rows: false,
      auto_select_columns: false,
      collapse_cell_selections: false,
      renderer: "SimpleCell".to_string(),
      grid_renderer: "by-columns-and-rows".to_string(),
      link: false,
      link_target: "_blank".to_string(),
      link_on_hover: false,
      link_color: "blue".to_string(),
      link_visited_color: "purple".to_string(),
      link_color_on_hover: false,
      strike_through: false,
      multiple_selections: false,
      enable_continuous_repaint: false,
      columns_reorderable: true,
      column_grab_margin: 5,
      column_clip: true,
      restore_row_selections: true,
      restore_column_selections: true,
      truncate_text_with_ellipsis: true,
    }
  }
}
