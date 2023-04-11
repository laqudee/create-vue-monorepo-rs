use std::path::PathBuf;
use crate::utils::render::render_template;

pub fn render(template_name: &str, template_root: &PathBuf, root: &PathBuf) -> std::io::Result<()> {
    let template_dir = template_root.join(template_name);
    render_template(&template_dir, root)?;

    Ok(())
}