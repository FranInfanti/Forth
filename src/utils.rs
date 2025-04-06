pub mod split {
    fn split_string(chars: &[char], i: &mut usize) -> String {
        let mut string = String::from(crate::START_STRING);

        *i += 2;
        loop {
            string = format!("{}{}", string, chars[*i]);
            if chars[*i] == '"' {
                break;
            }
            *i += 1;
        }
        *i += 1;

        string.trim().to_string()
    }

    /// Realiza un split del buf recibido por parametro,
    /// acorde a las necesidades del programa.
    /// Retorna un vector de String que contiene la separación deseada.
    ///
    /// # Errors
    ///
    /// # Examples
    ///
    /// ```
    ///     let buf = "1 2 + IF .\" Hallo  Welt\" THEN"
    ///     let cmd = split(buf);
    ///     // cmd = ["1", "2", "+", "IF", "." Hallo  Welt"", "THEN"]
    /// ```
    ///
    pub fn split(buf: &str) -> Vec<String> {
        let mut split = Vec::<String>::new();
        let chars: Vec<char> = buf.trim().chars().collect();

        let mut i = 0;
        while i < chars.len() {
            let mut string = String::new();
            while i < chars.len() && chars[i] != ' ' {
                if i + 1 < chars.len() && chars[i] == '.' && chars[i + 1] == '"' {
                    split.push(split_string(&chars, &mut i));
                    break;
                }

                string = format!("{}{}", string, chars[i].to_lowercase());
                i += 1;
            }

            if !string.is_empty() {
                split.push(string.trim().to_string());
            }

            while i < chars.len() && chars[i] == ' ' {
                i += 1;
            }
        }
        split
    }
}
