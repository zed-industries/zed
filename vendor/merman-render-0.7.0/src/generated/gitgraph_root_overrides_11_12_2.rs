// Fixture-derived root viewport overrides for Mermaid@11.12.2 GitGraph diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/gitgraph/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// They are used to keep `parity-root` stable at higher decimal precision when browser float
// behavior (DOM `getBBox()` + serialization) differs from our deterministic headless pipeline.

pub fn lookup_gitgraph_root_viewport_override(
    diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "upstream_html_demos_git_cherry_pick_from_branch_graph_013" => {
            Some(("-139.328125 -50 405.859375 203.3262939453125", "405.859375"))
        }
        "upstream_html_demos_git_cherry_pick_from_branch_graph_014" => Some((
            "-124.17623901367188 -50 347.265625 378.19744873046875",
            "347.265625",
        )),
        "upstream_html_demos_git_cherry_pick_from_branch_graph_015" => {
            Some(("-124.17623901367188 -50 347.265625 361", "347.265625"))
        }
        "upstream_html_demos_git_cherry_pick_from_main_graph_017" => Some((
            "-105.00030517578125 -50 331.5625 357.8128967285156",
            "331.5625",
        )),
        "upstream_merges_spec" => Some((
            "-146.375 -34.72539138793945 654.375 341.15484619140625",
            "654.375",
        )),
        "upstream_html_demos_git_simple_branch_and_merge_graph_001" => {
            Some(("-164.9375 -50 348.546875 204.290771484375", "348.546875"))
        }
        "upstream_html_demos_git_simple_branch_and_merge_graph_002" => {
            Some(("-134.10476684570312 -50 366.890625 238", "366.890625"))
        }
        "upstream_html_demos_git_simple_branch_and_merge_graph_003" => {
            Some(("-134.10476684570312 -50 366.890625 261", "366.890625"))
        }
        "upstream_html_demos_git_merge_feature_to_advanced_main_graph_007" => {
            Some(("-162.6953125 -50 394.0625 203.94400024414062", "394.0625"))
        }
        "upstream_html_demos_git_cherry_pick_from_main_graph_018" => {
            Some(("-105.00030517578125 -50 331.5625 361", "331.5625"))
        }
        "upstream_html_demos_git_merge_from_main_onto_undeveloped_branch_graph_022" => {
            Some(("-194.3203125 -50 479.21875 203.94400024414062", "479.21875"))
        }
        "upstream_html_demos_git_merge_from_main_onto_undeveloped_branch_graph_023" => {
            Some(("-204.80123901367188 -50 497.5625 288", "497.5625"))
        }
        "upstream_html_demos_git_merge_from_main_onto_undeveloped_branch_graph_024" => {
            Some(("-204.80123901367188 -50 497.5625 311", "497.5625"))
        }
        "upstream_html_demos_git_merge_from_main_onto_developed_branch_graph_025" => {
            Some(("-159.4921875 -50 459.5625 203.53219604492188", "459.5625"))
        }
        "upstream_html_demos_git_merge_from_main_onto_developed_branch_graph_026" => {
            Some(("-194.78854370117188 -50 477.890625 338", "477.890625"))
        }
        "upstream_html_demos_git_merge_from_main_onto_developed_branch_graph_027" => {
            Some(("-194.78854370117188 -50 477.890625 361", "477.890625"))
        }
        "upstream_cypress_gitgraph_spec_46_should_render_gitgraph_with_unconnected_branches_and_parallel_049" => {
            Some((
                "-62.7565803527832 -8 401.811279296875 354.42852783203125",
                "401.811279296875",
            ))
        }
        "stress_gitgraph_font_size_097" => Some((
            "-262.640625 -18 470.640625 177.95840454101562",
            "470.640625",
        )),
        "upstream_cypress_gitgraph_spec_31_should_render_a_simple_gitgraph_with_a_title_vertical_branch_034" => {
            Some((
                "-86.49547576904297 -50 143.734375 146.42852783203125",
                "143.734375",
            ))
        }
        "upstream_cypress_gitgraph_spec_43_should_render_gitgraph_with_parallel_commits_vertical_branch_046" => {
            Some((
                "-62.7565803527832 -8 331.006591796875 254.42852783203125",
                "331.006591796875",
            ))
        }
        "upstream_cypress_gitgraph_spec_4_should_render_a_simple_gitgraph_with_tags_committypes_on_main_004" => {
            Some(("-96 -34.70000076293945 254 145.16966247558594", "254"))
        }
        "upstream_cypress_gitgraph_spec_58_should_render_a_simple_gitgraph_with_rotated_labels_vertical_062" => {
            Some((
                "-179.8490447998047 22 213.3490447998047 338.87762451171875",
                "213.3490447998047",
            ))
        }
        "upstream_cypress_gitgraph_spec_71_should_render_gitgraph_with_parallel_commits_vertical_branch_075" => {
            Some((
                "-62.7565803527832 2 331.006591796875 239",
                "331.006591796875",
            ))
        }
        _ => None,
    }
}
