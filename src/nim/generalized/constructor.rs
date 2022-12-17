use std::{vec};

use super::{GeneralizedNimGame, closed_generalized::ClosedGeneralizedNimGame};

pub fn unit() -> ClosedGeneralizedNimGame{
    return ClosedGeneralizedNimGame::new(vec![vec![0]]);
}


pub fn rect(x: u16, y: u16) -> GeneralizedNimGame{
    let mut groups = vec![];
    for i in 0..x{
        let mut new_group = vec![];
        for j in 0..y{
            new_group.push(i+x*j);
        }
        groups.push(new_group);
    }
    for j in 0..y{
        let mut new_group = vec![];
        for i in 0..x{
            new_group.push(i+x*j);
        }
        groups.push(new_group);
    }
    return GeneralizedNimGame::new(groups);
}
pub fn triangle(l: u16) -> GeneralizedNimGame{
    let mut groups = vec![];
    for i in 0..l{
        let mut new_group1 = vec![];
        let mut new_group2 = vec![];
        let mut new_group3 = vec![];
        for j in 0..(l-i){


            /*
            12# # #
            8 9 # #
            4 5 6 #
            0 1 2 3
            */
            new_group1.push(i+j*l);
            new_group2.push(j+i*l);
            new_group3.push(l-1-i+j*(l-1));

        }
        groups.push(new_group1);
        groups.push(new_group2);
        groups.push(new_group3);
    }
    return GeneralizedNimGame::new(groups);
}
pub fn hyper_cube(dim: u16, l: u16) -> GeneralizedNimGame{
    let mut g = unit();
    for _ in 0..dim{
        g = extrude(g, l);
    }
    return g.into_generalized();
}
pub fn hyper_tetrahedron(dim: u16) -> GeneralizedNimGame{
    let mut g = unit();
    for _ in 0..dim{
        g = add_connection_to_all(g);
    }
    return g.into_generalized();
}
pub fn add_connection_to_all(g: ClosedGeneralizedNimGame) -> ClosedGeneralizedNimGame{
    let node_count = g.get_node_count();
    let mut groups = g.get_groups().clone();
    for i in 0..node_count{
        groups.push(vec![i,node_count]);
    }
    return ClosedGeneralizedNimGame::new(groups);
}
pub fn extrude(g: ClosedGeneralizedNimGame, l: u16) ->ClosedGeneralizedNimGame{
    let mut new_groups = g.get_groups().clone();

    for group in g.get_groups(){
        for offset in 0..l{
            let mut new_group = vec![];
            for node in group{
                new_group.push(node + offset*g.get_node_count());
            }
            new_groups.push(new_group);
        }
    }
    for node in 0..g.get_node_count(){
        let mut new_group = vec![];
        for offset in 0..l{
            new_group.push(node + offset*g.get_node_count());
        }
        new_groups.push(new_group);
    }

    return ClosedGeneralizedNimGame::new(new_groups);

}