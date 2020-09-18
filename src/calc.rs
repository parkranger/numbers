use crate::app::Result;
use crate::prime::prime_factorization;

#[derive(Debug, Default)]
pub struct Calc {
    pub values: Vec<u32>,
}

impl Calc {
    pub fn new(values: Vec<u32>) -> Self {
        Calc { values }
    }

    pub fn from_args(args: &clap::ArgMatches) -> Self {
        let values = args
            .values_of("values")
            .unwrap()
            .map(|x| x.parse().unwrap())
            .collect();
        Calc { values }
    }

    /// Formats the values as a comma separated list.
    fn values_for_display(&self) -> String {
        self.values
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Returns an array of prime factors for each value.
    fn values_pf(&self) -> Vec<Vec<u32>> {
        self.values
            .iter()
            .map(|i| prime_factorization(*i).unwrap())
            .collect()
    }

    pub fn command_pfz(&self) -> Result<()> {
        println!("Primfaktorzerlegung");

        for i in &self.values {
            match prime_factorization(*i) {
                Some(pf) => println!(" pfz({:?}) = {:?}", i, pf),
                None => println!(" pfz({:?}) = {{}}", i),
            }
        }

        Ok(())
    }

    /// Bestimmen des größten gemeinsamen Teilers (ggT) durch Primfaktorzerlegung.
    /// Man zerlegt die Zahlen jeweils in ihre Primfaktoren und bestimmt anschließend alle Faktoren,
    /// die in beiden Zerlegungen auftauchen. Diese Faktoren multipliziert man anschließend.
    /// Diese Methode läßt sich auch für den ggT von mehr als zwei Zahlen anwenden.
    ///
    /// Beispiel: Gesucht ist ggT(8, 20)
    ///
    /// ```
    ///     8 = 2 · 2 · 2
    ///    20 = 2 · 2 · 5
    ///   ---------------
    ///   ggT = 2 · 2     =  4
    /// ```
    ///
    /// Sollten die Zahlen keinen gemeinsamen Teiler besitzen, verwenden wir das Wort "teilerfremd".
    ///
    pub fn command_ggt(&self) -> Result<()> {
        let values_pf = self.values_pf();
        let mut ggt = 1;
        let mut pf_last: u32 = 0;

        for pf in &values_pf[0] {
            // process each factor only once. Requires a sorted list.
            if pf_last == *pf {
                continue;
            }
            pf_last = *pf;

            // Find the smallest number `n` of this prime number in each list.
            let n = values_pf
                .iter()
                .map(|v| v.iter().filter(|&&x| x == *pf).count())
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            // Multiply the prime number n times.
            (0..n).for_each(|_| ggt *= *pf)
        }

        if ggt == 1 {
            println!(" ggT({}) = {}", self.values_for_display(), "teilerfremd"); // coprime
        } else {
            println!(" ggT({}) = {}", self.values_for_display(), ggt);
        }

        Ok(())
    }

    /// Bestimmen des kleinsten gemainsamen Vilefachen (kgV) durch Primfaktorzerlegung.
    ///
    /// Als erstes bestimmt man die Primfaktorzerlegung der Zahlen. Anschließend fasst man
    /// alle auftretenden Primfaktoren in ihrer höchsten Anzahl zusammen.
    ///
    /// Beispiel: Gesucht ist kgV(8, 12)
    ///
    /// ```txt
    ///     8 = 2 · 2 · 2
    ///    12 = 2 · 2     · 3
    ///   -------------------
    ///   kgV = 2 · 2 · 2 · 3 = 24
    /// ```
    ///
    /// Der Faktor 2 tritt in der höchsten Anzahl 3 mal auf (bei der Zerlegung von 8).
    /// Der Faktor 3 tritt in der höchsten Anzahl 1 mal auf (bei der Zerlegung von 12).
    ///
    /// Anders als beim größten gemeinsamen Teiler (ggT) gibt es immer ein kgV.
    ///
    /// Beispiel: Gesucht ist kgV(6, 40)
    ///
    /// ```txt
    ///     6 = 2         · 3
    ///    40 = 2 · 2 · 2     · 5
    ///   -----------------------
    ///   kgV = 2 · 2 · 2 · 3 · 5 = 120
    /// ```
    ///
    pub fn command_kgv(&self) -> Result<()> {
        let mut values_pf = self.values_pf();
        let mut kgv = 1;

        while let Some(mut vec) = values_pf.pop() {
            while let Some(pf) = vec.pop() {
                kgv *= pf;
                // Remove one occurrence of this pf from each remaining vectors.
                values_pf.iter_mut().for_each(|vec2| {
                    if let Some(pos) = vec2.iter().position(|&x| x == pf) {
                        vec2.remove(pos);
                    }
                });
            }
        }

        println!(" kgV({}) = {}", self.values_for_display(), kgv);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_values_for_display() {
        let worker = Calc::new(vec![6, 40]);
        assert_eq!(worker.values_for_display(), "6, 40")
    }

    #[test]
    fn calc_values_pf() {
        let worker = Calc::new(vec![6, 40]);
        let mut res: Vec<Vec<u32>> = Vec::new();
        res.push(vec![2, 3]);
        res.push(vec![2, 2, 2, 5]);
        assert_eq!(worker.values_pf(), res);
    }

    #[test]
    fn calc_command_pfz() {
        let worker = Calc::new(vec![6, 40]);
        assert!(worker.command_pfz().is_ok());
    }

    #[test]
    fn calc_command_ggt() {
        let worker = Calc::new(vec![8, 20]);
        assert!(worker.command_ggt().is_ok()); // -> 4
    }

    #[test]
    fn calc_command_kgv() {
        let worker = Calc::new(vec![8, 12]);
        assert!(worker.command_kgv().is_ok()); // -> 24
    }
}
