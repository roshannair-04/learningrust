// 
// banking example struct
fn main(){
 let mut account=BankAccount{
    owner:"Alice".to_string(),
    balance:5000.0,
 };
 //immutable borrow to check the balance
 account.check_balance();
 //mutable to withdraw
 account.withdraw(45.50);
account.check_balance();

}
struct BankAccount{
    owner:String,
    balance:f64,
}
impl BankAccount{
    fn withdraw(&mut self, amount:f64){
        println!("withdrawing {} from account owned by {}",amount,self.owner);
        self.balance-=amount;
    }

    fn check_balance(&self){
        println!("account owned by {} has a balance {}",self.owner,self.balance);
    }
}