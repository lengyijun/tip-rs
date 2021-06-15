trait Term<T>{}

trait Var<T>: Term<T>{}
trait Cons<T>: Term<T>{}
trait Mu<T>: Term<T>{}

trait FreshVarType<T>: Var<T>{}
trait VarType<T>: Var<T>{}

trait IntType<T>: Cons<T>{}
trait FunctionType<T>: Cons<T>{}
trait PointerType<T>: Cons<T>{}
trait RecordType<T>: Cons<T>{}
trait AbsendType<T>: Cons<T>{}

trait RecursiveType<T>: Mu<T>{}
