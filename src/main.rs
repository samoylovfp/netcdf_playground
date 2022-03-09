fn main() -> anyhow::Result<()> {
    let mut f = netcdf::create("file.nc")?;
    f.add_dimension("lat", 3)?;
    f.add_dimension("lon", 3)?;
    {
        let mut lat = f.add_variable::<f32>("lat", &["lat"])?;
        lat.put_values(&[-90.0, 0.0, 90.0], None, None)?;
    }
    {
        let mut lon = f.add_variable::<f32>("lon", &["lon"])?;
        lon.put_values(&[0.0, 180.0, 359.0], None, None)?;
    }
    {
        let mut var = f.add_variable::<f32>("somevar", &["lat", "lon"])?;
        var.put_values(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], None, None)?;
    }

    Ok(())
}
