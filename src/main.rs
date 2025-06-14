fn ordemCrescente(lista: &mut Vec<i64>) {
    let n = lista.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && lista[j - 1] > lista[j] {
            lista.swap(j, j - 1);
            j -= 1;
        }

    }
}

fn filtrar_pares(vetor: &mut Vec<i64>) {
    let mut pares = Vec::new();

    for valor in vetor.iter() {
        if valor % 2 == 0 {
            pares.push(*valor);
        } 

    }

    *vetor = pares;
}

fn executar_estrategia(lista: &mut Vec<i64>, estrategia: fn(&mut Vec<i64>)) {
    estrategia(lista);
    println!("Resultado: {:?}", lista);
}

fn main() {
    let mut numeros : Vec<i64> = vec![5, 5, 1, 0, 9, 8, 6, 2, 8, 5, 2, 9, 1, 0, 4, 4];
    println!("Lista original: {:?}", numeros);

    executar_estrategia(&mut numeros, ordemCrescente);
    executar_estrategia(&mut numeros, filtrar_pares);

}
