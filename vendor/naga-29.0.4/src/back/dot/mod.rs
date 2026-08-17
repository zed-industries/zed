/*!
Backend for [DOT][dot] (Graphviz).

This backend writes a graph in the DOT language, for the ease
of IR inspection and debugging.

[dot]: https://graphviz.org/doc/info/lang.html
*/

use alloc::{
    borrow::Cow,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::{Error as FmtError, Write as _};

use crate::{
    arena::Handle,
    valid::{FunctionInfo, ModuleInfo},
};

/// Configuration options for the dot backend
#[derive(Clone, Default)]
pub struct Options {
    /// Only emit function bodies
    pub cfg_only: bool,
}

/// Identifier used to address a graph node
type NodeId = usize;

/// Stores the target nodes for control flow statements
#[derive(Default, Clone, Copy)]
struct Targets {
    /// The node, if some, where continue operations will land
    continue_target: Option<usize>,
    /// The node, if some, where break operations will land
    break_target: Option<usize>,
}

/// Stores information about the graph of statements
#[derive(Default)]
struct StatementGraph {
    /// List of node names
    nodes: Vec<&'static str>,
    /// List of edges of the control flow, the items are defined as
    /// (from, to, label)
    flow: Vec<(NodeId, NodeId, &'static str)>,
    /// List of implicit edges of the control flow, used for jump
    /// operations such as continue or break, the items are defined as
    /// (from, to, label, color_id)
    jumps: Vec<(NodeId, NodeId, &'static str, usize)>,
    /// List of dependency relationships between a statement node and
    /// expressions
    dependencies: Vec<(NodeId, Handle<crate::Expression>, &'static str)>,
    /// List of expression emitted by statement node
    emits: Vec<(NodeId, Handle<crate::Expression>)>,
    /// List of function call by statement node
    calls: Vec<(NodeId, Handle<crate::Function>)>,
}

impl StatementGraph {
    /// Adds a new block to the statement graph, returning the first and last node, respectively
    fn add(&mut self, block: &[crate::Statement], targets: Targets) -> (NodeId, NodeId) {
        use crate::Statement as S;

        // The first node of the block isn't a statement but a virtual node
        let root = self.nodes.len();
        self.nodes.push(if root == 0 { "Root" } else { "Node" });
        // Track the last placed node, this will be returned to the caller and
        // will also be used to generate the control flow edges
        let mut last_node = root;
        for statement in block {
            // Reserve a new node for the current statement and link it to the
            // node of the previous statement
            let id = self.nodes.len();
            self.flow.push((last_node, id, ""));
            self.nodes.push(""); // reserve space

            // Track the node identifier for the merge node, the merge node is
            // the last node of a statement, normally this is the node itself,
            // but for control flow statements such as `if`s and `switch`s this
            // is a virtual node where all branches merge back.
            let mut merge_id = id;

            self.nodes[id] = match *statement {
                S::Emit(ref range) => {
                    for handle in range.clone() {
                        self.emits.push((id, handle));
                    }
                    "Emit"
                }
                S::Kill => "Kill", //TODO: link to the beginning
                S::Break => {
                    // Try to link to the break target, otherwise produce
                    // a broken connection
                    if let Some(target) = targets.break_target {
                        self.jumps.push((id, target, "Break", 5))
                    } else {
                        self.jumps.push((id, root, "Broken", 7))
                    }
                    "Break"
                }
                S::Continue => {
                    // Try to link to the continue target, otherwise produce
                    // a broken connection
                    if let Some(target) = targets.continue_target {
                        self.jumps.push((id, target, "Continue", 5))
                    } else {
                        self.jumps.push((id, root, "Broken", 7))
                    }
                    "Continue"
                }
                S::ControlBarrier(_flags) => "ControlBarrier",
                S::MemoryBarrier(_flags) => "MemoryBarrier",
                S::Block(ref b) => {
                    let (other, last) = self.add(b, targets);
                    self.flow.push((id, other, ""));
                    // All following nodes should connect to the end of the block
                    // statement so change the merge id to it.
                    merge_id = last;
                    "Block"
                }
                S::If {
                    condition,
                    ref accept,
                    ref reject,
                } => {
                    self.dependencies.push((id, condition, "condition"));
                    let (accept_id, accept_last) = self.add(accept, targets);
                    self.flow.push((id, accept_id, "accept"));
                    let (reject_id, reject_last) = self.add(reject, targets);
                    self.flow.push((id, reject_id, "reject"));

                    // Create a merge node, link the branches to it and set it
                    // as the merge node to make the next statement node link to it
                    merge_id = self.nodes.len();
                    self.nodes.push("Merge");
                    self.flow.push((accept_last, merge_id, ""));
                    self.flow.push((reject_last, merge_id, ""));

                    "If"
                }
                S::Switch {
                    selector,
                    ref cases,
                } => {
                    self.dependencies.push((id, selector, "selector"));

                    // Create a merge node and set it as the merge node to make
                    // the next statement node link to it
                    merge_id = self.nodes.len();
                    self.nodes.push("Merge");

                    // Create a new targets structure and set the break target
                    // to the merge node
                    let mut targets = targets;
                    targets.break_target = Some(merge_id);

                    for case in cases {
                        let (case_id, case_last) = self.add(&case.body, targets);
                        let label = match case.value {
                            crate::SwitchValue::Default => "default",
                            _ => "case",
                        };
                        self.flow.push((id, case_id, label));
                        // Link the last node of the branch to the merge node
                        self.flow.push((case_last, merge_id, ""));
                    }
                    "Switch"
                }
                S::Loop {
                    ref body,
                    ref continuing,
                    break_if,
                } => {
                    // Create a new targets structure and set the break target
                    // to the merge node, this must happen before generating the
                    // continuing block since it can break.
                    let mut targets = targets;
                    targets.break_target = Some(id);

                    let (continuing_id, continuing_last) = self.add(continuing, targets);

                    // Set the the continue target to the beginning
                    // of the newly generated continuing block
                    targets.continue_target = Some(continuing_id);

                    let (body_id, body_last) = self.add(body, targets);

                    self.flow.push((id, body_id, "body"));

                    // Link the last node of the body to the continuing block
                    self.flow.push((body_last, continuing_id, "continuing"));
                    // Link the last node of the continuing block back to the
                    // beginning of the loop body
                    self.flow.push((continuing_last, body_id, "continuing"));

                    if let Some(expr) = break_if {
                        self.dependencies.push((continuing_id, expr, "break if"));
                    }

                    "Loop"
                }
                S::Return { value } => {
                    if let Some(expr) = value {
                        self.dependencies.push((id, expr, "value"));
                    }
                    "Return"
                }
                S::Store { pointer, value } => {
                    self.dependencies.push((id, value, "value"));
                    self.emits.push((id, pointer));
                    "Store"
                }
                S::ImageStore {
                    image,
                    coordinate,
                    array_index,
                    value,
                } => {
                    self.dependencies.push((id, image, "image"));
                    self.dependencies.push((id, coordinate, "coordinate"));
                    if let Some(expr) = array_index {
                        self.dependencies.push((id, expr, "array_index"));
                    }
                    self.dependencies.push((id, value, "value"));
                    "ImageStore"
                }
                S::Call {
                    function,
                    ref arguments,
                    result,
                } => {
                    for &arg in arguments {
                        self.dependencies.push((id, arg, "arg"));
                    }
                    if let Some(expr) = result {
                        self.emits.push((id, expr));
                    }
                    self.calls.push((id, function));
                    "Call"
                }
                S::Atomic {
                    pointer,
                    ref fun,
                    value,
                    result,
                } => {
                    if let Some(result) = result {
                        self.emits.push((id, result));
                    }
                    self.dependencies.push((id, pointer, "pointer"));
                    self.dependencies.push((id, value, "value"));
                    if let crate::AtomicFunction::Exchange { compare: Some(cmp) } = *fun {
                        self.dependencies.push((id, cmp, "cmp"));
                    }
                    "Atomic"
                }
                S::ImageAtomic {
                    image,
                    coordinate,
                    array_index,
                    fun: _,
                    value,
                } => {
                    self.dependencies.push((id, image, "image"));
                    self.dependencies.push((id, coordinate, "coordinate"));
                    if let Some(expr) = array_index {
                        self.dependencies.push((id, expr, "array_index"));
                    }
                    self.dependencies.push((id, value, "value"));
                    "ImageAtomic"
                }
                S::WorkGroupUniformLoad { pointer, result } => {
                    self.emits.push((id, result));
                    self.dependencies.push((id, pointer, "pointer"));
                    "WorkGroupUniformLoad"
                }
                S::RayQuery { query, ref fun } => {
                    self.dependencies.push((id, query, "query"));
                    match *fun {
                        crate::RayQueryFunction::Initialize {
                            acceleration_structure,
                            descriptor,
                        } => {
                            self.dependencies.push((
                                id,
                                acceleration_structure,
                                "acceleration_structure",
                            ));
                            self.dependencies.push((id, descriptor, "descriptor"));
                            "RayQueryInitialize"
                        }
                        crate::RayQueryFunction::Proceed { result } => {
                            self.emits.push((id, result));
                            "RayQueryProceed"
                        }
                        crate::RayQueryFunction::GenerateIntersection { hit_t } => {
                            self.dependencies.push((id, hit_t, "hit_t"));
                            "RayQueryGenerateIntersection"
                        }
                        crate::RayQueryFunction::ConfirmIntersection => {
                            "RayQueryConfirmIntersection"
                        }
                        crate::RayQueryFunction::Terminate => "RayQueryTerminate",
                    }
                }
                S::SubgroupBallot { result, predicate } => {
                    if let Some(predicate) = predicate {
                        self.dependencies.push((id, predicate, "predicate"));
                    }
                    self.emits.push((id, result));
                    "SubgroupBallot"
                }
                S::SubgroupCollectiveOperation {
                    op,
                    collective_op,
                    argument,
                    result,
                } => {
                    self.dependencies.push((id, argument, "arg"));
                    self.emits.push((id, result));
                    match (collective_op, op) {
                        (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::All) => {
                            "SubgroupAll"
                        }
                        (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Any) => {
                            "SubgroupAny"
                        }
                        (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Add) => {
                            "SubgroupAdd"
                        }
                        (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Mul) => {
                            "SubgroupMul"
                        }
                        (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Max) => {
                            "SubgroupMax"
                        }
                        (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Min) => {
                            "SubgroupMin"
                        }
                        (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::And) => {
                            "SubgroupAnd"
                        }
                        (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Or) => {
                            "SubgroupOr"
                        }
                        (crate::CollectiveOperation::Reduce, crate::SubgroupOperation::Xor) => {
                            "SubgroupXor"
                        }
                        (
                            crate::CollectiveOperation::ExclusiveScan,
                            crate::SubgroupOperation::Add,
                        ) => "SubgroupExclusiveAdd",
                        (
                            crate::CollectiveOperation::ExclusiveScan,
                            crate::SubgroupOperation::Mul,
                        ) => "SubgroupExclusiveMul",
                        (
                            crate::CollectiveOperation::InclusiveScan,
                            crate::SubgroupOperation::Add,
                        ) => "SubgroupInclusiveAdd",
                        (
                            crate::CollectiveOperation::InclusiveScan,
                            crate::SubgroupOperation::Mul,
                        ) => "SubgroupInclusiveMul",
                        _ => unimplemented!(),
                    }
                }
                S::SubgroupGather {
                    mode,
                    argument,
                    result,
                } => {
                    match mode {
                        crate::GatherMode::BroadcastFirst => {}
                        crate::GatherMode::Broadcast(index)
                        | crate::GatherMode::Shuffle(index)
                        | crate::GatherMode::ShuffleDown(index)
                        | crate::GatherMode::ShuffleUp(index)
                        | crate::GatherMode::ShuffleXor(index)
                        | crate::GatherMode::QuadBroadcast(index) => {
                            self.dependencies.push((id, index, "index"))
                        }
                        crate::GatherMode::QuadSwap(_) => {}
                    }
                    self.dependencies.push((id, argument, "arg"));
                    self.emits.push((id, result));
                    match mode {
                        crate::GatherMode::BroadcastFirst => "SubgroupBroadcastFirst",
                        crate::GatherMode::Broadcast(_) => "SubgroupBroadcast",
                        crate::GatherMode::Shuffle(_) => "SubgroupShuffle",
                        crate::GatherMode::ShuffleDown(_) => "SubgroupShuffleDown",
                        crate::GatherMode::ShuffleUp(_) => "SubgroupShuffleUp",
                        crate::GatherMode::ShuffleXor(_) => "SubgroupShuffleXor",
                        crate::GatherMode::QuadBroadcast(_) => "SubgroupQuadBroadcast",
                        crate::GatherMode::QuadSwap(direction) => match direction {
                            crate::Direction::X => "SubgroupQuadSwapX",
                            crate::Direction::Y => "SubgroupQuadSwapY",
                            crate::Direction::Diagonal => "SubgroupQuadSwapDiagonal",
                        },
                    }
                }
                S::CooperativeStore { target, data } => {
                    self.dependencies.push((id, target, "target"));
                    self.dependencies.push((id, data.pointer, "pointer"));
                    self.dependencies.push((id, data.stride, "stride"));
                    if data.row_major {
                        "CoopStoreT"
                    } else {
                        "CoopStore"
                    }
                }
                S::RayPipelineFunction(func) => match func {
                    crate::RayPipelineFunction::TraceRay {
                        acceleration_structure,
                        descriptor,
                        payload,
                    } => {
                        self.dependencies.push((
                            id,
                            acceleration_structure,
                            "acceleration_structure",
                        ));
                        self.dependencies.push((id, descriptor, "descriptor"));
                        self.dependencies.push((id, payload, "payload"));
                        "TraceRay"
                    }
                },
            };
            // Set the last node to the merge node
            last_node = merge_id;
        }
        (root, last_node)
    }
}

fn name(option: &Option<String>) -> &str {
    option.as_deref().unwrap_or_default()
}

/// set39 color scheme from <https://graphviz.org/doc/info/colors.html>
const COLORS: &[&str] = &[
    "white", // pattern starts at 1
    "#8dd3c7", "#ffffb3", "#bebada", "#fb8072", "#80b1d3", "#fdb462", "#b3de69", "#fccde5",
    "#d9d9d9",
];

struct Prefixed<T>(Handle<T>);

impl core::fmt::Display for Prefixed<crate::Expression> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.write_prefixed(f, "e")
    }
}

impl core::fmt::Display for Prefixed<crate::LocalVariable> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.write_prefixed(f, "l")
    }
}

impl core::fmt::Display for Prefixed<crate::GlobalVariable> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.write_prefixed(f, "g")
    }
}

impl core::fmt::Display for Prefixed<crate::Function> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.write_prefixed(f, "f")
    }
}

fn write_fun(
    output: &mut String,
    prefix: String,
    fun: &crate::Function,
    info: Option<&FunctionInfo>,
    options: &Options,
) -> Result<(), FmtError> {
    writeln!(output, "\t\tnode [ style=filled ]")?;

    if !options.cfg_only {
        for (handle, var) in fun.local_variables.iter() {
            writeln!(
                output,
                "\t\t{}_{} [ shape=hexagon label=\"{:?} '{}'\" ]",
                prefix,
                Prefixed(handle),
                handle,
                name(&var.name),
            )?;
        }

        write_function_expressions(output, &prefix, fun, info)?;
    }

    let mut sg = StatementGraph::default();
    sg.add(&fun.body, Targets::default());
    for (index, label) in sg.nodes.into_iter().enumerate() {
        writeln!(
            output,
            "\t\t{prefix}_s{index} [ shape=square label=\"{label}\" ]",
        )?;
    }
    for (from, to, label) in sg.flow {
        writeln!(
            output,
            "\t\t{prefix}_s{from} -> {prefix}_s{to} [ arrowhead=tee label=\"{label}\" ]",
        )?;
    }
    for (from, to, label, color_id) in sg.jumps {
        writeln!(
            output,
            "\t\t{}_s{} -> {}_s{} [ arrowhead=tee style=dashed color=\"{}\" label=\"{}\" ]",
            prefix, from, prefix, to, COLORS[color_id], label,
        )?;
    }

    if !options.cfg_only {
        for (to, expr, label) in sg.dependencies {
            writeln!(
                output,
                "\t\t{}_{} -> {}_s{} [ label=\"{}\" ]",
                prefix,
                Prefixed(expr),
                prefix,
                to,
                label,
            )?;
        }
        for (from, to) in sg.emits {
            writeln!(
                output,
                "\t\t{}_s{} -> {}_{} [ style=dotted ]",
                prefix,
                from,
                prefix,
                Prefixed(to),
            )?;
        }
    }

    assert!(sg.calls.is_empty());
    for (from, function) in sg.calls {
        writeln!(
            output,
            "\t\t{}_s{} -> {}_s0",
            prefix,
            from,
            Prefixed(function),
        )?;
    }

    Ok(())
}

fn write_function_expressions(
    output: &mut String,
    prefix: &str,
    fun: &crate::Function,
    info: Option<&FunctionInfo>,
) -> Result<(), FmtError> {
    enum Payload<'a> {
        Arguments(&'a [Handle<crate::Expression>]),
        Local(Handle<crate::LocalVariable>),
        Global(Handle<crate::GlobalVariable>),
    }

    let mut edges = crate::FastHashMap::<&str, _>::default();
    let mut payload = None;
    for (handle, expression) in fun.expressions.iter() {
        use crate::Expression as E;
        let (label, color_id) = match *expression {
            E::Literal(_) => ("Literal".into(), 2),
            E::Constant(_) => ("Constant".into(), 2),
            E::Override(_) => ("Override".into(), 2),
            E::ZeroValue(_) => ("ZeroValue".into(), 2),
            E::Compose { ref components, .. } => {
                payload = Some(Payload::Arguments(components));
                ("Compose".into(), 3)
            }
            E::Access { base, index } => {
                edges.insert("base", base);
                edges.insert("index", index);
                ("Access".into(), 1)
            }
            E::AccessIndex { base, index } => {
                edges.insert("base", base);
                (format!("AccessIndex[{index}]").into(), 1)
            }
            E::Splat { size, value } => {
                edges.insert("value", value);
                (format!("Splat{size:?}").into(), 3)
            }
            E::Swizzle {
                size,
                vector,
                pattern,
            } => {
                edges.insert("vector", vector);
                (format!("Swizzle{:?}", &pattern[..size as usize]).into(), 3)
            }
            E::FunctionArgument(index) => (format!("Argument[{index}]").into(), 1),
            E::GlobalVariable(h) => {
                payload = Some(Payload::Global(h));
                ("Global".into(), 2)
            }
            E::LocalVariable(h) => {
                payload = Some(Payload::Local(h));
                ("Local".into(), 1)
            }
            E::Load { pointer } => {
                edges.insert("pointer", pointer);
                ("Load".into(), 4)
            }
            E::ImageSample {
                image,
                sampler,
                gather,
                coordinate,
                array_index,
                offset: _,
                level,
                depth_ref,
                clamp_to_edge: _,
            } => {
                edges.insert("image", image);
                edges.insert("sampler", sampler);
                edges.insert("coordinate", coordinate);
                if let Some(expr) = array_index {
                    edges.insert("array_index", expr);
                }
                match level {
                    crate::SampleLevel::Auto => {}
                    crate::SampleLevel::Zero => {}
                    crate::SampleLevel::Exact(expr) => {
                        edges.insert("level", expr);
                    }
                    crate::SampleLevel::Bias(expr) => {
                        edges.insert("bias", expr);
                    }
                    crate::SampleLevel::Gradient { x, y } => {
                        edges.insert("grad_x", x);
                        edges.insert("grad_y", y);
                    }
                }
                if let Some(expr) = depth_ref {
                    edges.insert("depth_ref", expr);
                }
                let string = match gather {
                    Some(component) => Cow::Owned(format!("ImageGather{component:?}")),
                    _ => Cow::Borrowed("ImageSample"),
                };
                (string, 5)
            }
            E::ImageLoad {
                image,
                coordinate,
                array_index,
                sample,
                level,
            } => {
                edges.insert("image", image);
                edges.insert("coordinate", coordinate);
                if let Some(expr) = array_index {
                    edges.insert("array_index", expr);
                }
                if let Some(sample) = sample {
                    edges.insert("sample", sample);
                }
                if let Some(level) = level {
                    edges.insert("level", level);
                }
                ("ImageLoad".into(), 5)
            }
            E::ImageQuery { image, query } => {
                edges.insert("image", image);
                let args = match query {
                    crate::ImageQuery::Size { level } => {
                        if let Some(expr) = level {
                            edges.insert("level", expr);
                        }
                        Cow::from("ImageSize")
                    }
                    _ => Cow::Owned(format!("{query:?}")),
                };
                (args, 7)
            }
            E::Unary { op, expr } => {
                edges.insert("expr", expr);
                (format!("{op:?}").into(), 6)
            }
            E::Binary { op, left, right } => {
                edges.insert("left", left);
                edges.insert("right", right);
                (format!("{op:?}").into(), 6)
            }
            E::Select {
                condition,
                accept,
                reject,
            } => {
                edges.insert("condition", condition);
                edges.insert("accept", accept);
                edges.insert("reject", reject);
                ("Select".into(), 3)
            }
            E::Derivative { axis, ctrl, expr } => {
                edges.insert("", expr);
                (format!("d{axis:?}{ctrl:?}").into(), 8)
            }
            E::Relational { fun, argument } => {
                edges.insert("arg", argument);
                (format!("{fun:?}").into(), 6)
            }
            E::Math {
                fun,
                arg,
                arg1,
                arg2,
                arg3,
            } => {
                edges.insert("arg", arg);
                if let Some(expr) = arg1 {
                    edges.insert("arg1", expr);
                }
                if let Some(expr) = arg2 {
                    edges.insert("arg2", expr);
                }
                if let Some(expr) = arg3 {
                    edges.insert("arg3", expr);
                }
                (format!("{fun:?}").into(), 7)
            }
            E::As {
                kind,
                expr,
                convert,
            } => {
                edges.insert("", expr);
                let string = match convert {
                    Some(width) => format!("Convert<{kind:?},{width}>"),
                    None => format!("Bitcast<{kind:?}>"),
                };
                (string.into(), 3)
            }
            E::CallResult(_function) => ("CallResult".into(), 4),
            E::AtomicResult { .. } => ("AtomicResult".into(), 4),
            E::WorkGroupUniformLoadResult { .. } => ("WorkGroupUniformLoadResult".into(), 4),
            E::ArrayLength(expr) => {
                edges.insert("", expr);
                ("ArrayLength".into(), 7)
            }
            E::RayQueryProceedResult => ("rayQueryProceedResult".into(), 4),
            E::RayQueryGetIntersection { query, committed } => {
                edges.insert("", query);
                let ty = if committed { "Committed" } else { "Candidate" };
                (format!("rayQueryGet{ty}Intersection").into(), 4)
            }
            E::SubgroupBallotResult => ("SubgroupBallotResult".into(), 4),
            E::SubgroupOperationResult { .. } => ("SubgroupOperationResult".into(), 4),
            E::RayQueryVertexPositions { query, committed } => {
                edges.insert("", query);
                let ty = if committed { "Committed" } else { "Candidate" };
                (format!("get{ty}HitVertexPositions").into(), 4)
            }
            E::CooperativeLoad { ref data, .. } => {
                edges.insert("pointer", data.pointer);
                edges.insert("stride", data.stride);
                let suffix = if data.row_major { "T " } else { "" };
                (format!("coopLoad{suffix}").into(), 4)
            }
            E::CooperativeMultiplyAdd { a, b, c } => {
                edges.insert("a", a);
                edges.insert("b", b);
                edges.insert("c", c);
                ("cooperativeMultiplyAdd".into(), 4)
            }
        };

        // give uniform expressions an outline
        let color_attr = match info {
            Some(info) if info[handle].uniformity.non_uniform_result.is_none() => "fillcolor",
            _ => "color",
        };
        writeln!(
            output,
            "\t\t{}_{} [ {}=\"{}\" label=\"{:?} {}\" ]",
            prefix,
            Prefixed(handle),
            color_attr,
            COLORS[color_id],
            handle,
            label,
        )?;

        for (key, edge) in edges.drain() {
            writeln!(
                output,
                "\t\t{}_{} -> {}_{} [ label=\"{}\" ]",
                prefix,
                Prefixed(edge),
                prefix,
                Prefixed(handle),
                key,
            )?;
        }
        match payload.take() {
            Some(Payload::Arguments(list)) => {
                write!(output, "\t\t{{")?;
                for &comp in list {
                    write!(output, " {}_{}", prefix, Prefixed(comp))?;
                }
                writeln!(output, " }} -> {}_{}", prefix, Prefixed(handle))?;
            }
            Some(Payload::Local(h)) => {
                writeln!(
                    output,
                    "\t\t{}_{} -> {}_{}",
                    prefix,
                    Prefixed(h),
                    prefix,
                    Prefixed(handle),
                )?;
            }
            Some(Payload::Global(h)) => {
                writeln!(
                    output,
                    "\t\t{} -> {}_{} [fillcolor=gray]",
                    Prefixed(h),
                    prefix,
                    Prefixed(handle),
                )?;
            }
            None => {}
        }
    }

    Ok(())
}

/// Write shader module to a [`String`].
pub fn write(
    module: &crate::Module,
    mod_info: Option<&ModuleInfo>,
    options: Options,
) -> Result<String, FmtError> {
    use core::fmt::Write as _;

    let mut output = String::new();
    output += "digraph Module {\n";

    if !options.cfg_only {
        writeln!(output, "\tsubgraph cluster_globals {{")?;
        writeln!(output, "\t\tlabel=\"Globals\"")?;
        for (handle, var) in module.global_variables.iter() {
            writeln!(
                output,
                "\t\t{} [ shape=hexagon label=\"{:?} {:?}/'{}'\" ]",
                Prefixed(handle),
                handle,
                var.space,
                name(&var.name),
            )?;
        }
        writeln!(output, "\t}}")?;
    }

    for (handle, fun) in module.functions.iter() {
        let prefix = Prefixed(handle).to_string();
        writeln!(output, "\tsubgraph cluster_{prefix} {{")?;
        writeln!(
            output,
            "\t\tlabel=\"Function{:?}/'{}'\"",
            handle,
            name(&fun.name)
        )?;
        let info = mod_info.map(|a| &a[handle]);
        write_fun(&mut output, prefix, fun, info, &options)?;
        writeln!(output, "\t}}")?;
    }
    for (ep_index, ep) in module.entry_points.iter().enumerate() {
        let prefix = format!("ep{ep_index}");
        writeln!(output, "\tsubgraph cluster_{prefix} {{")?;
        writeln!(output, "\t\tlabel=\"{:?}/'{}'\"", ep.stage, ep.name)?;
        let info = mod_info.map(|a| a.get_entry_point(ep_index));
        write_fun(&mut output, prefix, &ep.function, info, &options)?;
        writeln!(output, "\t}}")?;
    }

    output += "}\n";
    Ok(output)
}
