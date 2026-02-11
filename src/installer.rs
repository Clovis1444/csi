pub struct Installer {
    program_name: String,
    pages: Vec<Box<dyn InstallerPage>>,
}

pub enum InstallerPageType {
    Page1,
    Page2,
}
impl From<&str> for InstallerPageType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "page1" => InstallerPageType::Page1,
            "page2" => InstallerPageType::Page2,
            invalid_val => {
                panic!(
                    "Can't convert {invalid_val} to {}.",
                    std::any::type_name::<InstallerPageType>()
                )
            }
        }
    }
}
impl std::fmt::Display for InstallerPageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallerPageType::Page1 => write!(f, "Page1"),
            InstallerPageType::Page2 => write!(f, "Page2"),
        }
    }
}

trait InstallerPage {
    fn title(&self) -> &str;
    fn index(&self) -> &str;
    fn page_type(&self) -> InstallerPageType;
}
