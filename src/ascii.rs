//! Paysage nocturne en ASCII (étoiles, lune, montagnes, sapins, brume), généré au démarrage.
//! Le tirage est déterministe : la même graine donne toujours le même paysage.
//!
//! Le résultat est du HTML : un `<pre>` dont les caractères sont regroupés par couche dans des
//! `<span>` (la couleur vient de base.css), plus des bandes de brume qui dérivent par-dessus.

use std::fmt::Write;

/// Largeur du paysage, en caractères. Large pour couvrir les grands écrans.
const COLS: usize = 300;
/// Hauteur du paysage, en lignes.
const ROWS: usize = 34;

pub struct SceneOptions {
    pub seed: u64,
    /// Position de la lune, en fraction de la largeur.
    pub moon_x: f32,
    /// Rayon de la lune, en lignes (un caractère est deux fois plus haut que large).
    pub moon_r: i32,
}

/// Générateur pseudo-aléatoire minimal (xorshift64*).
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn float(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() % (hi - lo) as u64) as i32
    }
}

/// Couche d'un caractère : elle donne sa classe CSS, donc sa couleur.
#[derive(Clone, Copy, PartialEq)]
enum Layer {
    Empty,
    Star,
    Twinkle,
    Moon,
    /// Crêtes, de la plus lointaine (0) à la plus proche (2).
    Ridge(u8),
}

impl Layer {
    fn class(self) -> &'static str {
        match self {
            Layer::Empty => "",
            Layer::Star => "s",
            Layer::Twinkle => "t",
            Layer::Moon => "m",
            Layer::Ridge(0) => "r1",
            Layer::Ridge(1) => "r2",
            Layer::Ridge(_) => "r3",
        }
    }
}

struct Grid(Vec<[(char, Layer); COLS]>);

impl Grid {
    fn new() -> Self {
        Grid(vec![[(' ', Layer::Empty); COLS]; ROWS])
    }
    fn set(&mut self, col: i32, row: i32, c: char, layer: Layer) {
        if (0..COLS as i32).contains(&col) && (0..ROWS as i32).contains(&row) {
            self.0[row as usize][col as usize] = (c, layer);
        }
    }
    /// Le texte, ligne par ligne, avec un `<span>` par suite de caractères de la même couche.
    fn html(&self, out: &mut String) {
        for line in &self.0 {
            let mut current = Layer::Empty;
            // Les espaces en fin de ligne ne servent à rien.
            let end = line.iter().rposition(|&(c, _)| c != ' ').map_or(0, |i| i + 1);
            for &(c, layer) in &line[..end] {
                // Un espace ne change pas de couche : il reste dans le `<span>` en cours.
                let layer = if c == ' ' { current } else { layer };
                if layer != current {
                    if current != Layer::Empty {
                        out.push_str("</span>");
                    }
                    if layer != Layer::Empty {
                        let _ = write!(out, r#"<span class="{}">"#, layer.class());
                    }
                    current = layer;
                }
                out.push(c);
            }
            if current != Layer::Empty {
                out.push_str("</span>");
            }
            out.push('\n');
        }
    }
}

/// Ligne de crête (numéro de ligne depuis le haut) pour chaque colonne.
fn ridge(rng: &mut Rng, base: f32, amp: f32, freq: f32) -> Vec<i32> {
    let (p1, p2, p3) = (rng.float() * 6.3, rng.float() * 6.3, rng.float() * 6.3);
    (0..COLS)
        .map(|c| {
            let x = c as f32 * freq;
            let wave =
                0.55 * (x + p1).sin() + 0.3 * (x * 2.3 + p2).sin() + 0.15 * (x * 5.1 + p3).sin();
            (base - amp * wave).round() as i32
        })
        .collect()
}

/// Une bande de brume : des tirets et des tildes par paquets. Le motif est écrit deux fois de
/// suite, pour que l'animation (un décalage d'une demi-largeur) boucle sans raccord visible.
fn fog_lines(rng: &mut Rng, rows: usize, density: f32) -> String {
    let mut out = String::new();
    for _ in 0..rows {
        let mut line = String::with_capacity(COLS);
        while line.len() < COLS {
            if rng.float() < density {
                let c = ['~', '~', '-'][rng.range(0, 3) as usize];
                for _ in 0..rng.range(3, 14) {
                    line.push(if rng.float() < 0.15 { ' ' } else { c });
                }
            } else {
                for _ in 0..rng.range(4, 22) {
                    line.push(' ');
                }
            }
        }
        line.truncate(COLS);
        out.push_str(&line);
        out.push_str(&line);
        out.push('\n');
    }
    out
}

/// Bande de brume seule (sans paysage), posée entre les sections et au-dessus du pied de page.
pub fn fog_strip(seed: u64) -> String {
    let mut rng = Rng(seed | 1);
    format!(
        r#"<pre class="fogstrip" aria-hidden="true">{}</pre>"#,
        fog_lines(&mut rng, 4, 0.4)
    )
}

pub fn render(opt: &SceneOptions) -> String {
    let mut rng = Rng(opt.seed | 1);
    let mut grid = Grid::new();

    // Étoiles
    for _ in 0..170 {
        let (col, row) = (rng.range(0, COLS as i32), rng.range(0, 20));
        let c = ['.', '.', '.', '\'', '+', '*'][rng.range(0, 6) as usize];
        let layer = if rng.float() < 0.25 { Layer::Twinkle } else { Layer::Star };
        grid.set(col, row, c, layer);
    }

    // Lune : un disque plein, avec un bord plus clair et quelques cratères.
    let (mx, my, r) = ((opt.moon_x * COLS as f32) as i32, 8, opt.moon_r.max(2));
    for dy in -r..=r {
        for dx in -2 * r..=2 * r {
            let d = ((dx as f32 / 2.0).powi(2) + (dy as f32).powi(2)).sqrt() / r as f32;
            if d > 1.0 {
                continue;
            }
            let crater = [(-4, -2), (-3, -2), (3, 1), (4, 1), (-1, 2), (5, -2)].contains(&(dx, dy));
            let c = if crater {
                'o'
            } else if d > 0.82 {
                '%'
            } else {
                '@'
            };
            grid.set(mx + dx, my + dy, c, Layer::Moon);
        }
    }

    // Crêtes de l'arrière vers l'avant : chacune efface ce qu'elle recouvre.
    let layers: [(f32, f32, f32); 3] = [(17.0, 5.0, 0.03), (22.0, 4.0, 0.045), (27.0, 3.0, 0.06)];
    let mut fog = String::new();
    for (i, (base, amp, freq)) in layers.iter().enumerate() {
        let layer = Layer::Ridge(i as u8);
        let tops = ridge(&mut rng, *base, *amp, *freq);
        for (c, &top) in tops.iter().enumerate() {
            let left = if c > 0 { tops[c - 1] } else { top };
            let right = tops.get(c + 1).copied().unwrap_or(top);
            // Le trait de crête : il monte, il descend, ou il reste à plat (tracé une ligne
            // plus haut, car `_` est en bas de sa case).
            let (row, outline) = match (top < left, top < right) {
                (true, true) => (top, '^'),
                (true, false) => (top, '/'),
                (false, true) => (top, '\\'),
                (false, false) => (top - 1, '_'),
            };
            // Sous la crête : du vide (qui masque les étoiles) et quelques points épars.
            for row in top - 1..ROWS as i32 {
                let dot = row > top && rng.float() < 0.02;
                grid.set(c as i32, row, if dot { '.' } else { ' ' }, layer);
            }
            grid.set(c as i32, row, outline, layer);
        }

        // Sapins sur les deux crêtes les plus proches.
        if i >= 1 {
            let mut c = rng.range(2, 10);
            while c < COLS as i32 - 3 {
                let top = tops[c as usize];
                let tall = rng.range(2, 4);
                grid.set(c, top - tall, '^', layer);
                for row in 1..tall {
                    grid.set(c - 1, top - tall + row, '/', layer);
                    grid.set(c, top - tall + row, '|', layer);
                    grid.set(c + 1, top - tall + row, '\\', layer);
                }
                c += rng.range(7, 26);
            }
        }

        // Un banc de brume qui dérive au pied de chaque crête, chacun à sa vitesse.
        let _ = write!(
            fog,
            r#"<pre class="fog fog{i}" style="--row:{}">{}</pre>"#,
            *base as i32 + 1,
            fog_lines(&mut rng, 2, 0.22 + 0.06 * i as f32)
        );
    }

    let mut html = String::with_capacity(24 * 1024);
    html.push_str(r#"<div class="scene" aria-hidden="true"><pre class="land">"#);
    grid.html(&mut html);
    html.push_str("</pre>");
    html.push_str(&fog);
    html.push_str("</div>");
    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_and_valid() {
        let opt = SceneOptions {
            seed: 42,
            moon_x: 0.7,
            moon_r: 4,
        };
        let a = render(&opt);
        assert_eq!(a, render(&opt));
        assert!(a.starts_with("<div") && a.ends_with("</div>"));
        assert!(a.contains(r#"class="m""#) && a.contains(r#"class="fog fog0""#));
        // Autant de balises ouvertes que fermées, et aucun caractère à échapper.
        assert_eq!(a.matches("<span").count(), a.matches("</span>").count());
        let f = fog_strip(3);
        assert_eq!(f, fog_strip(3));
        assert!(f.starts_with("<pre") && f.ends_with("</pre>") && f.contains('~'));
    }

    #[test]
    fn land_is_a_rectangle() {
        let mut grid = Grid::new();
        grid.set(0, 0, '*', Layer::Star);
        grid.set(-1, 0, '*', Layer::Star);
        grid.set(0, ROWS as i32, '*', Layer::Star);
        let mut out = String::new();
        grid.html(&mut out);
        assert_eq!(out.lines().count(), ROWS);
        assert_eq!(out.lines().next(), Some(r#"<span class="s">*</span>"#));
    }
}
