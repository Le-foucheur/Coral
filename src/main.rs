use core::fmt;
use std::{env, io::{stdout, Write}, ops::{Index, IndexMut}, thread::sleep, time::Duration};
use libc::{exit};
use owo_colors::OwoColorize;
use terminal_size::{terminal_size, Width, Height};
use ctrlc;
use colorgrad::Gradient;

const TEMPS: Duration = Duration::from_millis(0); // Default 70

#[derive(Clone, Debug)]
struct Matrice<T> {
    largeur: usize,
    hauteur: usize,
    tableau: Vec<T>,
    nb_state: u8,
}  

impl<T: Clone> Matrice<T> {
    fn new (largeur: usize, hauteur: usize, val_init: T, nb_state: u8) -> Self {
        Self {
            largeur: largeur,
            hauteur: 2 * hauteur,
            tableau: vec![val_init; largeur * 2 * hauteur],
            nb_state: nb_state,
        }

    }
}

impl Matrice<u8> {
    fn gen_ran (&mut self) -> () {
        for y in 0..self.hauteur {
            for x in 0..self.largeur {
                self[(x,y)] = rand::random_range(0..self.nb_state);
            }
        }
    }
}

impl<T> Index<(usize, usize)> for Matrice<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x,y) = index;
        &self.tableau[self.largeur * y + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrice<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x,y) = index;
        &mut self.tableau[self.largeur * y + x]
    }
}

impl fmt::Display for Matrice<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let grad = colorgrad::preset::rainbow();
        write!(f, "\x1b[1;1H")?;
        for y in (0..self.hauteur).step_by(2) {
            for x in 0..self.largeur {
                let [ir,iv,ib,_] = grad.at((self[(x,y)] as f32) / (self.nb_state as f32)).to_rgba8();
                let [jr,jv,jb,_] = grad.at((self[(x,y + 1)] as f32) / (self.nb_state as f32)).to_rgba8();
                write!(f, "{}", "▀".truecolor(ir,iv,ib).on_truecolor(jr, jv,jb))?;
            }
        }
        Ok(())
    }
}

fn coovois (i: isize, x: isize, y:isize) -> (isize, isize) {
    (x + (i%3) - 1, y + ((i/3)%3) - 1)
}

fn tempsp1 (mat:&mut Matrice<u8>) -> () {
    let tmp = mat.clone();
    for x in 0..tmp.largeur {
        for y in 0..tmp.hauteur {
            let mut nbcell = vec![0; mat.nb_state as usize];
            for i in 0..9 {
                if i != 4 {
                    let (x,y) = coovois(i, x as isize, y as isize);
                    if x < 0 || x as usize >= tmp.largeur || y < 0 || y as usize >= tmp.hauteur {
                        ()
                    }
                    else {
                        let type_cell = tmp[(x as usize,y as usize)] as usize;
                        nbcell[type_cell] += 1;
                    }
                }
            }
            for i in 0..mat.nb_state {
                // println!("s: {} - nbcell: {:?} - i: {} - mod: {}", mat.nb_state, nbcell, i, ((i + 1) % (mat.nb_state)) as usize);
                if tmp[(x,y)] == i  && nbcell[((i + 1) % (mat.nb_state)) as usize] >= 3 {
                    mat[(x,y)] = (i + 1) % mat.nb_state 
                }
            }
        }
    }
}

fn main () {

    // cache le curseur
    print!("\x1b[?47h\x1b[?1049h\x1b[?25l");

    // sortie ctr + c «propre»
    ctrlc::set_handler(move || {print!("\x1b[?1049l\x1b[?25h\x1b[?47l");
    let _ = stdout().flush(); 
    unsafe { exit(0) }; }).expect("T’Con");

    let (Width(w),Height(h)) = terminal_size().unwrap();
    let w:usize = w.into();
    let h:usize = (h).into();

    // Récupère le paramètre de scale
    let args: Vec<String> = env::args().collect();
    let nb_state: u8 = args[1].parse::<u8>().unwrap();

    let mut a = Matrice::new(w, h, 0 as u8, nb_state);

    if args.len() >= 3 {
        a.gen_ran();    
    }

    //a[(w, h * 3 /2)] = 3;
    //a[(w - 1, h * 3 /2)] = 3;
    //a[(w + 1, h * 3 /2)] = 3;
    //
    //a[(1,0)] = 2;
    //a[(2,1)] = 2;
    //a[(0,2)] = 2;
    //a[(1,2)] = 2;
    //a[(2,2)] = 2;
    //
    //a[(1 + 96,0)] = 1;
    //a[(2 + 96,1)] = 1;
    //a[(0 + 96,2)] = 1;
    //a[(1 + 96,2)] = 1;
    //a[(2 + 96,2)] = 1;

    loop {
        let now = std::time::Instant::now();
        print!("{a}");
        tempsp1(&mut a);
        let dif = now.elapsed();
        if dif <= TEMPS {
            sleep(TEMPS - dif);
        }
    }


}
