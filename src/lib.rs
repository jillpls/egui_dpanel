use egui::{Context, Frame, InnerResponse, SidePanel, TopBottomPanel, Ui};

pub enum PanelCfg {
    Single(SinglePanelCfg),
    Collapsible(CollapsiblePanelCfg),
}

impl PanelCfg {
    pub fn collapsed(&self) -> &SinglePanelCfg {
        match self {
            PanelCfg::Single(s) => s,
            PanelCfg::Collapsible(c) => &c.collapsed,
        }
    }

    pub fn expanded(&self) -> &SinglePanelCfg {
        match self {
            PanelCfg::Single(s) => s,
            PanelCfg::Collapsible(c) => &c.expanded,
        }
    }
}

pub struct CollapsiblePanelCfg {
    pub collapsed: SinglePanelCfg,
    pub expanded: SinglePanelCfg,
}

impl CollapsiblePanelCfg {
    pub fn new(collapsed: SinglePanelCfg, expanded: SinglePanelCfg) -> Self {
        Self {
            collapsed,
            expanded,
        }
    }
}

pub struct SinglePanelCfg {
    side: Side,
    pub resizable: Option<bool>,
    pub show_separator_line: Option<bool>,
    pub default_width: Option<f32>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub width_range: Option<(f32, f32)>,
    pub exact_width: Option<f32>,
    pub default_height: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub height_range: Option<(f32, f32)>,
    pub exact_height: Option<f32>,
    pub frame: Option<Frame>,
}

impl Into<PanelCfg> for SinglePanelCfg {
    fn into(self) -> PanelCfg {
        PanelCfg::Single(self)
    }
}

impl SinglePanelCfg {
    pub fn left() -> Self {
        Self::new(Side::Left)
    }

    pub fn right() -> Self {
        Self::new(Side::Right)
    }

    pub fn top() -> Self {
        Self::new(Side::Top)
    }

    pub fn bottom() -> Self {
        Self::new(Side::Bottom)
    }

    pub fn new(side: Side) -> Self {
        Self {
            side,
            resizable: None,
            show_separator_line: None,
            default_width: None,
            min_width: None,
            max_width: None,
            width_range: None,
            exact_width: None,
            default_height: None,
            min_height: None,
            max_height: None,
            height_range: None,
            exact_height: None,
            frame: None,
        }
    }

    fn side(&self) -> Side {
        self.side
    }

    pub fn apply_top_bottom(&self, panel: TopBottomPanel) -> TopBottomPanel {
        let panel = if let Some(b) = self.resizable {
            panel.resizable(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.show_separator_line {
            panel.show_separator_line(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.default_height {
            panel.default_height(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.min_height {
            panel.min_height(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.max_height {
            panel.max_height(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.height_range {
            panel.height_range(b.0..=b.1)
        } else {
            panel
        };
        let panel = if let Some(b) = self.exact_height {
            panel.exact_height(b)
        } else {
            panel
        };
        if let Some(f) = self.frame {
            panel.frame(f)
        } else {
            panel
        }
    }

    pub fn apply_side(&self, panel: SidePanel) -> SidePanel {
        let panel = if let Some(b) = self.resizable {
            panel.resizable(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.show_separator_line {
            panel.show_separator_line(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.default_width {
            panel.default_width(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.min_width {
            panel.min_width(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.max_width {
            panel.max_width(b)
        } else {
            panel
        };
        let panel = if let Some(b) = self.width_range {
            panel.width_range(b.0..=b.1)
        } else {
            panel
        };
        let panel = if let Some(b) = self.exact_width {
            panel.exact_width(b)
        } else {
            panel
        };
        if let Some(f) = self.frame {
            panel.frame(f)
        } else {
            panel
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Side {
    pub fn is_lr(&self) -> bool {
        match self {
            Side::Left | Side::Right => true,
            _ => false,
        }
    }
}

pub struct DynamicPanel {
    name: String,
    panels: Vec<PanelCfg>,
    choice_f: Option<Box<dyn Fn(&egui::Context) -> usize>>,
}

impl DynamicPanel {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            panels: vec![],
            choice_f: None,
        }
    }
    pub fn show_dynamic<R, F: Fn(&mut egui::Ui) -> R>(
        &self,
        ctx: &egui::Context,
        content: F,
    ) -> Option<egui::InnerResponse<R>> {
        self.choice_f
            .as_ref()
            .and_then(|f| self.show(ctx, (f)(ctx), content))
    }

    pub fn show_dynamic_inside<R, F: Fn(&mut egui::Ui) -> R>(
        &self,
        ctx: &Context,
        ui: &mut Ui,
        content: F,
    ) -> Option<egui::InnerResponse<R>> {
        self.choice_f
            .as_ref()
            .and_then(|f| self.show_inside(ui, f(ctx), content))
    }

    pub fn show_dynamic_animated<R, F: Fn(&mut egui::Ui) -> R>(
        &self,
        ctx: &egui::Context,
        is_expanded: bool,
        content: F,
    ) -> Option<egui::InnerResponse<R>> {
        self.choice_f
            .as_ref()
            .and_then(|f| self.show_animated(ctx, f(ctx), is_expanded, content))
    }

    pub fn show_dynamic_animated_inside<R, F: Fn(&mut egui::Ui) -> R>(
        &self,
        ctx: &egui::Context,
        ui: &mut Ui,
        is_expanded: bool,
        content: F,
    ) -> Option<egui::InnerResponse<R>> {
        self.choice_f
            .as_ref()
            .and_then(|f| self.show_animated_inside(ui, f(ctx), is_expanded, content))
    }

    pub fn show<R, F: Fn(&mut egui::Ui) -> R>(
        &self,
        ctx: &egui::Context,
        index: usize,
        content: F,
    ) -> Option<egui::InnerResponse<R>> {
        if let Some(cfg) = self.panels.get(index) {
            Some(Self::show_panel(
                cfg.expanded(),
                ctx,
                content,
                self.name.clone(),
            ))
        } else {
            None
        }
    }

    pub fn show_inside<R, F: Fn(&mut egui::Ui) -> R>(
        &self,
        ui: &mut Ui,
        index: usize,
        content: F,
    ) -> Option<egui::InnerResponse<R>> {
        if let Some(cfg) = self.panels.get(index) {
            Some(Self::show_panel_inside(
                cfg.expanded(),
                ui,
                content,
                self.name.clone(),
            ))
        } else {
            None
        }
    }

    pub fn show_animated<R, F: Fn(&mut egui::Ui) -> R>(
        &self,
        ctx: &egui::Context,
        index: usize,
        is_expanded: bool,
        content: F,
    ) -> Option<egui::InnerResponse<R>> {
        if let Some(cfg) = self.panels.get(index) {
            Self::show_panel_animated(cfg.expanded(), ctx, is_expanded, content, self.name.clone())
        } else {
            None
        }
    }

    pub fn show_animated_inside<R, F: Fn(&mut egui::Ui) -> R>(
        &self,
        ui: &mut Ui,
        index: usize,
        is_expanded: bool,
        content: F,
    ) -> Option<egui::InnerResponse<R>> {
        if let Some(cfg) = self.panels.get(index) {
            Self::show_panel_animated_inside(
                cfg.expanded(),
                ui,
                is_expanded,
                content,
                self.name.clone(),
            )
        } else {
            None
        }
    }
}

impl DynamicPanel {
    pub fn dual(mut self, first : PanelCfg, second: PanelCfg) -> Self {
        self.panels = vec![first, second];
        self
    }

    pub fn with_threshold_function<F: Fn(&Context) -> bool>(mut self, f : F) -> Self {
        let f = |ctx| { if f(ctx) { 1 } else { 0 }};
        self.choice_f = Some(Box::new(f));
        self
    }

    pub fn with_panels(mut self, panels: Vec<PanelCfg>) -> Self {
        self.panels = panels;
        self
    }

    pub fn push_panel(&mut self, panel: PanelCfg) -> usize {
        let index = self.panels.len();
        self.panels.push(panel);
        index
    }

    pub fn with_choice_function<F: Fn(&Context) -> usize + 'static>(
        mut self,
        choice_function: F,
    ) -> Self {
        self.choice_f = Some(Box::new(choice_function));
        self
    }
}

impl DynamicPanel {
    fn build_side_panel(cfg: &SinglePanelCfg, name: impl Into<egui::Id>) -> SidePanel {
        let side = if cfg.side == Side::Left {
            egui::panel::Side::Left
        } else {
            egui::panel::Side::Right
        };
        let panel = SidePanel::new(side, name);
        cfg.apply_side(panel)
    }

    fn build_top_bottom_panel(cfg: &SinglePanelCfg, name: impl Into<egui::Id>) -> TopBottomPanel {
        let side = if cfg.side == Side::Top {
            egui::panel::TopBottomSide::Top
        } else {
            egui::panel::TopBottomSide::Bottom
        };
        let panel = TopBottomPanel::new(side, name);
        cfg.apply_top_bottom(panel)
    }

    fn show_panel<R, F: Fn(&mut egui::Ui) -> R>(
        cfg: &SinglePanelCfg,
        ctx: &Context,
        content: F,
        name: impl Into<egui::Id>,
    ) -> egui::InnerResponse<R> {
        match cfg.side {
            Side::Left | Side::Right => {
                let panel = Self::build_side_panel(cfg, name);
                panel.show(ctx, content)
            }
            Side::Top | Side::Bottom => {
                let panel = Self::build_top_bottom_panel(cfg, name);
                panel.show(ctx, content)
            }
        }
    }

    fn show_panel_inside<R, F: Fn(&mut egui::Ui) -> R>(
        cfg: &SinglePanelCfg,
        ui: &mut Ui,
        content: F,
        name: impl Into<egui::Id>,
    ) -> egui::InnerResponse<R> {
        match cfg.side {
            Side::Left | Side::Right => {
                let panel = Self::build_side_panel(cfg, name);
                panel.show_inside(ui, content)
            }
            Side::Top | Side::Bottom => {
                let panel = Self::build_top_bottom_panel(cfg, name);
                panel.show_inside(ui, content)
            }
        }
    }

    fn show_panel_animated<R, F: Fn(&mut egui::Ui) -> R>(
        cfg: &SinglePanelCfg,
        ctx: &Context,
        is_expanded: bool,
        content: F,
        name: impl Into<egui::Id>,
    ) -> Option<egui::InnerResponse<R>> {
        match cfg.side {
            Side::Left | Side::Right => {
                let panel = Self::build_side_panel(cfg, name);
                panel.show_animated(ctx, is_expanded, content)
            }
            Side::Top | Side::Bottom => {
                let panel = Self::build_top_bottom_panel(cfg, name);
                panel.show_animated(ctx, is_expanded, content)
            }
        }
    }

    fn show_panel_animated_inside<R, F: Fn(&mut egui::Ui) -> R>(
        cfg: &SinglePanelCfg,
        ui: &mut Ui,
        is_expanded: bool,
        content: F,
        name: impl Into<egui::Id>,
    ) -> Option<egui::InnerResponse<R>> {
        match cfg.side {
            Side::Left | Side::Right => {
                let panel = Self::build_side_panel(cfg, name);
                panel.show_animated_inside(ui, is_expanded, content)
            }
            Side::Top | Side::Bottom => {
                let panel = Self::build_top_bottom_panel(cfg, name);
                panel.show_animated_inside(ui, is_expanded, content)
            }
        }
    }

    fn show_panel_animated_between<R, F: Fn(&mut Ui, f32) -> R>(
        cfg: &PanelCfg,
        ctx: &Context,
        is_expanded: bool,
        content: F,
        name: impl Into<egui::Id> + Clone,
    ) -> Option<InnerResponse<R>> {
        match (
            cfg.collapsed().side().is_lr(),
            cfg.expanded().side().is_lr(),
        ) {
            (true, true) => {
                let collapsed = Self::build_side_panel(cfg.collapsed(), name.clone());
                let expanded = Self::build_side_panel(cfg.expanded(), name);
                SidePanel::show_animated_between(ctx, is_expanded, collapsed, expanded, content)
            }
            (false, false) => {
                let collapsed = Self::build_top_bottom_panel(cfg.collapsed(), name.clone());
                let expanded = Self::build_top_bottom_panel(cfg.expanded(), name);
                TopBottomPanel::show_animated_between(
                    ctx,
                    is_expanded,
                    collapsed,
                    expanded,
                    content,
                )
            }
            (_, _) => None,
        }
    }

    fn show_panel_animated_between_inside<R, F: Fn(&mut Ui, f32) -> R>(
        cfg: &PanelCfg,
        ui: &mut Ui,
        is_expanded: bool,
        content: F,
        name: impl Into<egui::Id> + Clone,
    ) -> Option<InnerResponse<R>> {
        match (
            cfg.collapsed().side().is_lr(),
            cfg.expanded().side().is_lr(),
        ) {
            (true, true) => {
                let collapsed = Self::build_side_panel(cfg.collapsed(), name.clone());
                let expanded = Self::build_side_panel(cfg.expanded(), name);
                Some(SidePanel::show_animated_between_inside(
                    ui,
                    is_expanded,
                    collapsed,
                    expanded,
                    content,
                ))
            }
            (false, false) => {
                let collapsed = Self::build_top_bottom_panel(cfg.collapsed(), name.clone());
                let expanded = Self::build_top_bottom_panel(cfg.expanded(), name);
                Some(TopBottomPanel::show_animated_between_inside(
                    ui,
                    is_expanded,
                    collapsed,
                    expanded,
                    content,
                ))
            }
            _ => None,
        }
    }
}
