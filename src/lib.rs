// Importa os módulos std::env e std::fs
use std::{env, fs};

// Estrutura Config que armazena as configurações da pesquisa
pub struct Config {
    pub query: String, // A consulta a ser pesquisada
    pub file_path: String, // O caminho do arquivo onde a pesquisa será realizada
    pub ignore_case: bool, // Se a pesquisa deve ignorar a diferença entre maiúsculas e minúsculas
}

// Implementação da estrutura Config
impl Config{
    // Método para construir uma nova configuração
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Verifica se há argumentos suficientes
        if args.len() < 3 {
            return Err("não há argumentos suficientes");
        }

        let query = args[1].clone(); // A consulta
        let file_path = args[2].clone(); // O caminho do arquivo
        let ignore_case = env::var("IGNORE_CASE").is_ok(); // Se deve ignorar a diferença entre maiúsculas e minúsculas

        // Retorna a nova configuração
        Ok(Config { query, file_path, ignore_case })
    }
}

// Função para executar a pesquisa
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Lê o conteúdo do arquivo
    let contents = fs::read_to_string(config.file_path)?;

    // Realiza a pesquisa, considerando ou não a diferença entre maiúsculas e minúsculas
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // Imprime os resultados da pesquisa
    for line in results {
        println!("{}", line);
    }

    // Retorna Ok se tudo ocorreu bem
    Ok(())
}

// Função para realizar a pesquisa considerando a diferença entre maiúsculas e minúsculas
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); // Vetor para armazenar os resultados

    // Percorre cada linha do conteúdo
    for line in contents.lines() {
        // Se a linha contém a consulta, adiciona ao vetor de resultados
        if line.contains(query) {
            results.push(line);
        }
    }

    // Retorna os resultados
    results
}

// Função para realizar a pesquisa ignorando a diferença entre maiúsculas e minúsculas
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // Converte a consulta para minúsculas
    let mut results = Vec::new(); // Vetor para armazenar os resultados

    // Percorre cada linha do conteúdo
    for line in contents.lines() {
        // Se a linha contém a consulta (ignorando a diferença entre maiúsculas e minúsculas), adiciona ao vetor de resultados
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    // Retorna os resultados
    results
}

// Testes para as funções de pesquisa
#[cfg(test)]
mod tests {
    use super::*;

    // Teste para a pesquisa considerando a diferença entre maiúsculas e minúsculas
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // Teste para a pesquisa ignorando a diferença entre maiúsculas e minúsculas
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}