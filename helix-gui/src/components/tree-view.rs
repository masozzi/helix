#[derive(Default)]
struct TreeView {
    root: std::path::Path,
}

enum TreeViewMessage {
    OpenFile(std::fs::File),
    OpenDir(std::fs::File),
}

impl TreeView {
    pub fn new(root: std::path::Path) -> Self {
        return Self { root };
    }
}

impl<'a, TreeViewMessage, Renderer> From<TreeView> for iced::Element<'a, TreeViewMessage, Renderer>
    where Renderer: iced::advanced::renderer::Renderer,
{
    fn from(value: TreeView) -> Self {
        return Self::new(value);
    }
}
