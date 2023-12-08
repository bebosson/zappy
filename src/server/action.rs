pub mod action{

    enum State{
        Idle,
        Wait,
        Action,
    }
    enum EnumAction{
        Mouvement,
        Info,
        Death
    }
    
    pub struct Action{
        state: State,
        count: u16,  
        timestamp: u64,
        action: EnumAction,
    }
    // enum Mouvement{
        //     Avance,
    //     Droite,
    //     Gauche,
    // }

    // enum Info{
    //     Voir,
    //     Inventaire,
    // }

    // //enum ...
    

    // enum 

    // pub trait Proceed{
    //     // pub fn proceed
    // }

    // impl Proceed for Mouvement{
    //     //...
    // }

    // impl Proceed for Info{
    //     //...
    // }

    // impl Proceed for Action{
    //     match Action{
    //         (Mouvement | Info) => x.proceed();
    //         (Death) => {

    //         }
    //     }
    // }

    // pub fn toto(p_proceed: &impl Proceed){

    // }

    

}