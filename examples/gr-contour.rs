use gr::gr;
use std::error::Error;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let x = &[0.0, 1.0];
    let y = &[0.0, 1.0];
    let z = &[0.0, 1.0, 1.0, 0.0];
    let y2 = &[0.0, 0.25, 0.5, 0.75, 1.0];
    let z2 = &[0.0, 1.4, 0.35, 1.15, 0.8, 0.8, 1.15, 0.35, 1.4, 0.0];

    gr::contourf(x, y, gr::GrContourHeights::N(30), z, None, true)?;
    gr::axes((0.1, 0.1), (0.0, 0.0), (None, Some(2)), -0.01)?;
    gr::updatews();
    stdin().bytes().next();

    gr::clearws();
    gr::axes3d((0.1, 0.1, 0.1), (0.0, 0.0, 0.0), (2, 0, 1), 0.02)?;
    gr::contour(x, y, None, z, 2, true)?;
    gr::updatews();
    stdin().bytes().next();

    gr::clearws();
    let (z_rng, _, _) = gr::inqspace();
    gr::setspace(z_rng, 0, 90)?;
    gr::contour(x, y2, Some(&[0.15, 0.35, 0.5, 0.65, 0.8, 0.9]), z2, 2, true)?;
    gr::axes((0.1, 0.1), (0.0, 0.0), (None, Some(2)), -0.01)?;
    gr::updatews();
    stdin().bytes().next();

    Ok(())
}
