use crate::{
    caja::Caja,
    golpe::{
        Golpe, constante_media::ConstanteMedia, cuadrilatero::Cuadrilatero, esfera::Esfera,
        golpeable::Golpeable, lista_golpeable::ListaGolpeable, nodo::Nodo, rotar_y::RotarY,
        trasladar::Trasladar,
    },
    intervalo::Intervalo,
    rayo::Rayo,
};

pub enum Objeto {
    Esfera(Esfera),
    Lista(ListaGolpeable),
    Cuadrilatero(Cuadrilatero),
    ConstanteMedia(ConstanteMedia),
    Nodo(Nodo),
    RotarY(RotarY),
    Trasladar(Trasladar),
}

impl Golpeable for Objeto {
    fn caja(&self) -> Caja {
        match self {
            Objeto::Esfera(e) => e.caja(),
            Objeto::Lista(l) => l.caja(),
            Objeto::Cuadrilatero(c) => c.caja(),
            Objeto::ConstanteMedia(cm) => cm.caja(),
            Objeto::Nodo(n) => n.caja(),
            Objeto::RotarY(ry) => ry.caja(),
            Objeto::Trasladar(t) => t.caja(),
        }
    }

    fn rayo_golpea(&self, rayo: &Rayo, intervalo: Intervalo) -> Option<Golpe> {
        match self {
            Objeto::Esfera(e) => e.rayo_golpea(rayo, intervalo),
            Objeto::Lista(l) => l.rayo_golpea(rayo, intervalo),
            Objeto::Cuadrilatero(c) => c.rayo_golpea(rayo, intervalo),
            Objeto::ConstanteMedia(cm) => cm.rayo_golpea(rayo, intervalo),
            Objeto::Nodo(n) => n.rayo_golpea(rayo, intervalo),
            Objeto::RotarY(ry) => ry.rayo_golpea(rayo, intervalo),
            Objeto::Trasladar(t) => t.rayo_golpea(rayo, intervalo),
        }
    }
}
