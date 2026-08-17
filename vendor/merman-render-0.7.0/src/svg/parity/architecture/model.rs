#[derive(Clone, Copy)]
pub(super) struct ArchitectureServiceRef<'a> {
    pub(super) id: &'a str,
    pub(super) icon: Option<&'a str>,
    pub(super) icon_text: Option<&'a str>,
    pub(super) title: Option<&'a str>,
    pub(super) in_group: Option<&'a str>,
}

#[derive(Clone, Copy)]
pub(super) struct ArchitectureJunctionRef<'a> {
    pub(super) id: &'a str,
    pub(super) in_group: Option<&'a str>,
}

#[derive(Clone, Copy)]
pub(super) struct ArchitectureGroupRef<'a> {
    pub(super) id: &'a str,
    pub(super) icon: Option<&'a str>,
    pub(super) title: Option<&'a str>,
    pub(super) in_group: Option<&'a str>,
}

#[derive(Clone, Copy)]
pub(super) struct ArchitectureEdgeRef<'a> {
    pub(super) lhs_id: &'a str,
    pub(super) lhs_dir: char,
    pub(super) lhs_into: Option<bool>,
    pub(super) lhs_group: Option<bool>,
    pub(super) rhs_id: &'a str,
    pub(super) rhs_dir: char,
    pub(super) rhs_into: Option<bool>,
    pub(super) rhs_group: Option<bool>,
    pub(super) title: Option<&'a str>,
}

pub(super) trait ArchitectureModelAccess {
    type Groups<'a>: Iterator<Item = ArchitectureGroupRef<'a>>
    where
        Self: 'a;
    type Services<'a>: Iterator<Item = ArchitectureServiceRef<'a>>
    where
        Self: 'a;
    type Junctions<'a>: Iterator<Item = ArchitectureJunctionRef<'a>>
    where
        Self: 'a;
    type Edges<'a>: Iterator<Item = ArchitectureEdgeRef<'a>>
    where
        Self: 'a;

    fn acc_title(&self) -> Option<&str>;
    fn acc_descr(&self) -> Option<&str>;

    fn groups_len(&self) -> usize;
    fn edges_len(&self) -> usize;

    fn groups(&self) -> Self::Groups<'_>;
    fn services(&self) -> Self::Services<'_>;
    fn junctions(&self) -> Self::Junctions<'_>;
    fn edges(&self) -> Self::Edges<'_>;
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ArchitectureService {
    id: String,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default, rename = "iconText")]
    icon_text: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default, rename = "in")]
    in_group: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ArchitectureJunction {
    id: String,
    #[serde(default, rename = "in")]
    in_group: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ArchitectureGroup {
    id: String,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default, rename = "in")]
    in_group: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ArchitectureEdge {
    #[serde(rename = "lhsId")]
    lhs_id: String,
    #[serde(rename = "lhsDir")]
    lhs_dir: char,
    #[serde(default, rename = "lhsInto")]
    lhs_into: Option<bool>,
    #[serde(default, rename = "lhsGroup")]
    lhs_group: Option<bool>,
    #[serde(rename = "rhsId")]
    rhs_id: String,
    #[serde(rename = "rhsDir")]
    rhs_dir: char,
    #[serde(default, rename = "rhsInto")]
    rhs_into: Option<bool>,
    #[serde(default, rename = "rhsGroup")]
    rhs_group: Option<bool>,
    #[serde(default)]
    title: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ArchitectureModel {
    #[serde(default, rename = "accTitle")]
    acc_title: Option<String>,
    #[serde(default, rename = "accDescr")]
    acc_descr: Option<String>,
    #[serde(default)]
    groups: Vec<ArchitectureGroup>,
    #[serde(default)]
    services: Vec<ArchitectureService>,
    #[serde(default)]
    junctions: Vec<ArchitectureJunction>,
    #[serde(default)]
    edges: Vec<ArchitectureEdge>,
}

pub(super) struct JsonGroupsIter<'a> {
    iter: std::slice::Iter<'a, ArchitectureGroup>,
}

impl<'a> Iterator for JsonGroupsIter<'a> {
    type Item = ArchitectureGroupRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|g| ArchitectureGroupRef {
            id: g.id.as_str(),
            icon: g.icon.as_deref(),
            title: g.title.as_deref(),
            in_group: g.in_group.as_deref(),
        })
    }
}

pub(super) struct JsonServicesIter<'a> {
    iter: std::slice::Iter<'a, ArchitectureService>,
}

impl<'a> Iterator for JsonServicesIter<'a> {
    type Item = ArchitectureServiceRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|s| ArchitectureServiceRef {
            id: s.id.as_str(),
            icon: s.icon.as_deref(),
            icon_text: s.icon_text.as_deref(),
            title: s.title.as_deref(),
            in_group: s.in_group.as_deref(),
        })
    }
}

pub(super) struct JsonJunctionsIter<'a> {
    iter: std::slice::Iter<'a, ArchitectureJunction>,
}

impl<'a> Iterator for JsonJunctionsIter<'a> {
    type Item = ArchitectureJunctionRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|j| ArchitectureJunctionRef {
            id: j.id.as_str(),
            in_group: j.in_group.as_deref(),
        })
    }
}

pub(super) struct JsonEdgesIter<'a> {
    iter: std::slice::Iter<'a, ArchitectureEdge>,
}

impl<'a> Iterator for JsonEdgesIter<'a> {
    type Item = ArchitectureEdgeRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| ArchitectureEdgeRef {
            lhs_id: e.lhs_id.as_str(),
            lhs_dir: e.lhs_dir,
            lhs_into: e.lhs_into,
            lhs_group: e.lhs_group,
            rhs_id: e.rhs_id.as_str(),
            rhs_dir: e.rhs_dir,
            rhs_into: e.rhs_into,
            rhs_group: e.rhs_group,
            title: e.title.as_deref(),
        })
    }
}

impl ArchitectureModelAccess for ArchitectureModel {
    type Groups<'a>
        = JsonGroupsIter<'a>
    where
        Self: 'a;
    type Services<'a>
        = JsonServicesIter<'a>
    where
        Self: 'a;
    type Junctions<'a>
        = JsonJunctionsIter<'a>
    where
        Self: 'a;
    type Edges<'a>
        = JsonEdgesIter<'a>
    where
        Self: 'a;

    fn acc_title(&self) -> Option<&str> {
        self.acc_title.as_deref()
    }

    fn acc_descr(&self) -> Option<&str> {
        self.acc_descr.as_deref()
    }

    fn groups_len(&self) -> usize {
        self.groups.len()
    }

    fn edges_len(&self) -> usize {
        self.edges.len()
    }

    fn groups(&self) -> Self::Groups<'_> {
        JsonGroupsIter {
            iter: self.groups.iter(),
        }
    }

    fn services(&self) -> Self::Services<'_> {
        JsonServicesIter {
            iter: self.services.iter(),
        }
    }

    fn junctions(&self) -> Self::Junctions<'_> {
        JsonJunctionsIter {
            iter: self.junctions.iter(),
        }
    }

    fn edges(&self) -> Self::Edges<'_> {
        JsonEdgesIter {
            iter: self.edges.iter(),
        }
    }
}

pub(super) struct TypedGroupsIter<'a> {
    iter: std::slice::Iter<'a, merman_core::diagrams::architecture::ArchitectureRenderGroup>,
}

impl<'a> Iterator for TypedGroupsIter<'a> {
    type Item = ArchitectureGroupRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|g| ArchitectureGroupRef {
            id: g.id.as_str(),
            icon: g.icon.as_deref(),
            title: g.title.as_deref(),
            in_group: g.in_group.as_deref(),
        })
    }
}

pub(super) struct TypedServicesIter<'a> {
    iter: std::slice::Iter<'a, merman_core::diagrams::architecture::ArchitectureRenderNode>,
}

impl<'a> Iterator for TypedServicesIter<'a> {
    type Item = ArchitectureServiceRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for n in self.iter.by_ref() {
            if n.node_type
                != merman_core::diagrams::architecture::ArchitectureRenderNodeType::Service
            {
                continue;
            }
            return Some(ArchitectureServiceRef {
                id: n.id.as_str(),
                icon: n.icon.as_deref(),
                icon_text: n.icon_text.as_deref(),
                title: n.title.as_deref(),
                in_group: n.in_group.as_deref(),
            });
        }
        None
    }
}

pub(super) struct TypedJunctionsIter<'a> {
    iter: std::slice::Iter<'a, merman_core::diagrams::architecture::ArchitectureRenderNode>,
}

impl<'a> Iterator for TypedJunctionsIter<'a> {
    type Item = ArchitectureJunctionRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for n in self.iter.by_ref() {
            if n.node_type
                != merman_core::diagrams::architecture::ArchitectureRenderNodeType::Junction
            {
                continue;
            }
            return Some(ArchitectureJunctionRef {
                id: n.id.as_str(),
                in_group: n.in_group.as_deref(),
            });
        }
        None
    }
}

pub(super) struct TypedEdgesIter<'a> {
    iter: std::slice::Iter<'a, merman_core::diagrams::architecture::ArchitectureRenderEdge>,
}

impl<'a> Iterator for TypedEdgesIter<'a> {
    type Item = ArchitectureEdgeRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| ArchitectureEdgeRef {
            lhs_id: e.lhs_id.as_str(),
            lhs_dir: e.lhs_dir,
            lhs_into: e.lhs_into,
            lhs_group: e.lhs_group,
            rhs_id: e.rhs_id.as_str(),
            rhs_dir: e.rhs_dir,
            rhs_into: e.rhs_into,
            rhs_group: e.rhs_group,
            title: e.title.as_deref(),
        })
    }
}

impl ArchitectureModelAccess
    for merman_core::diagrams::architecture::ArchitectureDiagramRenderModel
{
    type Groups<'a>
        = TypedGroupsIter<'a>
    where
        Self: 'a;
    type Services<'a>
        = TypedServicesIter<'a>
    where
        Self: 'a;
    type Junctions<'a>
        = TypedJunctionsIter<'a>
    where
        Self: 'a;
    type Edges<'a>
        = TypedEdgesIter<'a>
    where
        Self: 'a;

    fn acc_title(&self) -> Option<&str> {
        self.acc_title.as_deref()
    }

    fn acc_descr(&self) -> Option<&str> {
        self.acc_descr.as_deref()
    }

    fn groups_len(&self) -> usize {
        self.groups.len()
    }

    fn edges_len(&self) -> usize {
        self.edges.len()
    }

    fn groups(&self) -> Self::Groups<'_> {
        TypedGroupsIter {
            iter: self.groups.iter(),
        }
    }

    fn services(&self) -> Self::Services<'_> {
        TypedServicesIter {
            iter: self.nodes.iter(),
        }
    }

    fn junctions(&self) -> Self::Junctions<'_> {
        TypedJunctionsIter {
            iter: self.nodes.iter(),
        }
    }

    fn edges(&self) -> Self::Edges<'_> {
        TypedEdgesIter {
            iter: self.edges.iter(),
        }
    }
}
