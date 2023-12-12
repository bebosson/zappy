pub mod action
{
    #[derive(Debug)]
    enum State
    {
        Idle,
        Wait,
        Action,
    }
    #[derive(Debug)]
    enum EnumAction
    {
        Noaction,
        Mouvement,
        Info,
        Death
    }
    #[derive(Debug)]
    pub struct Action
    {
        state: State,
        count: u16,  
        timestamp: u64,
        action: EnumAction,
    }

    impl Action
    {
        pub fn new() -> Self
        {
            Action
            {
                state: State::Idle,
                count: 0,
                timestamp: 0,
                action: EnumAction::Noaction,
            }
        }
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