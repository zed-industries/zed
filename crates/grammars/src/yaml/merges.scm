; Auto-Resolve structural merge rules for YAML.
;
; Block-mapping keys identify each pair, like JSON.

(block_mapping) @merge.set

(flow_mapping) @merge.set

(block_sequence) @merge.ordered_list

(flow_sequence) @merge.ordered_list

(block_mapping_pair key: (_) @merge.key)

(flow_pair key: (_) @merge.key)
