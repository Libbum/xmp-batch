extern crate failure;
extern crate globwalk;
extern crate rexiv2;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate url;
extern crate url_serde;

use failure::Error;
use globwalk::DirEntry;
use std::fs::File;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Attribution {
    marked: bool,
    usage_terms: String,
    #[serde(with = "url_serde")]
    web_statement: Url,
    #[serde(with = "url_serde")]
    license: Url,
    #[serde(with = "url_serde")]
    more_permissions: Url,
    #[serde(with = "url_serde")]
    attribution_url: Url,
    attribution_name: String,
}

fn main() -> Result<(), Error> {

    let attribution_file = File::open("attribution.yaml")?;
    let attrib: Attribution = serde_yaml::from_reader(attribution_file)?;

    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        "images",
        &["*.{png,jpg,jpeg,PNG,JPG,JPEG}", "!*_small*", "!*_blur*", "!*_thumb*"], //Don't worry about thumbnails
    )
    .follow_links(true)
    .build()?
    .into_iter()
    .filter_map(Result::ok)
    .collect::<Vec<DirEntry>>();

    for file in walker.iter() {
        println!("Writing metadata to: {}", file.path().display());
        let meta = rexiv2::Metadata::new_from_path(&file.path())?;
        //Blanket clear all xmp data. TODO: this needs a better solution.
        meta.clear_xmp();
        rexiv2::unregister_all_xmp_namespaces();
        rexiv2::register_xmp_namespace("http://creativecommons.org/ns#/", "cc")?;

        let marked = match attrib.marked {
            true => "True",
            false => "False",
        };

        meta.set_tag_string("Xmp.xmpRights.Marked", marked)?;
        meta.set_tag_string("Xmp.xmpRights.UsageTerms", &attrib.usage_terms)?;
        meta.set_tag_string("Xmp.dc.rights", &attrib.usage_terms)?;
        meta.set_tag_string("Xmp.xmpRights.WebStatement", attrib.web_statement.as_str())?;
        meta.set_tag_string("Xmp.cc.license", attrib.license.as_str())?;
        meta.set_tag_string("Xmp.cc.morePermissions", attrib.more_permissions.as_str())?;
        meta.set_tag_string("Xmp.cc.attributionURL", attrib.attribution_url.as_str())?;
        meta.set_tag_string("Xmp.cc.attributionName", &attrib.attribution_name)?;

        meta.save_to_file(&file.path())?;
    }

    Ok(())
}
