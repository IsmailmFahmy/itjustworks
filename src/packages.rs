

pub enum InstallMethodEnum {
    Git,
    Targz
}


pub struct Package<'a> {
    pub name: &'a str,
    pub sourcecode_link: &'a str,
    pub instal_method : InstallMethodEnum
}
