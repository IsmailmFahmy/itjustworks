pub enum InstallMethodEnum {
    MakeInstall,
    AutoGen,
    
}

pub enum ExtractMethodEnum {
    Targz,
    Tarxz,
    Zip
}


pub struct Package<'a> {
    pub name: &'a str,
    pub sourcecode_link: &'a str,
    pub extract_method : ExtractMethodEnum,
    pub instal_method : InstallMethodEnum,
    pub service: Option<&'a str>,
}
