use diesel::{prelude::*, result::Error};

use crate::{
    establish_connection,
    models::{Region, Rom, RomRegion, RomWithRegion},
    schemas::*,
};

pub fn get_roms_with_region() -> Result<Vec<RomWithRegion>, Error> {
    let connection = &mut establish_connection();

    let roms = roms_table::table
        .select(Rom::as_select())
        .load(connection)?;

    let regions = RomRegion::belonging_to(&roms)
        .inner_join(regions_table::table)
        .select((RomRegion::as_select(), Region::as_select()))
        .load(connection)?;

    let roms_with_regions :Vec<RomWithRegion>= regions.grouped_by(&roms)
    .into_iter().zip(roms)
    .map(|(r, rom)| 
    RomWithRegion { rom, regions: r.into_iter().map(|(_, region)| region).collect() }
  )
    .collect::<Vec<RomWithRegion>>();

    Ok(roms_with_regions)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test() {
        let results = get_roms_with_region().expect("error getting roms");
        let json = serde_json::to_value(&results).unwrap();


        
        println!("{:#?}", json);
    }
}