```
from wiki

--Closure--
In programming languages, closures (also lexical closures or function closures) are a technique for implementing lexically scoped name binding in languages with first-class functions. Operationally, a closure is a data structure storing a function[a] together with an environment:[1] a mapping associating each free variable of the function (variables that are used locally, but defined in an enclosing scope) with the value or storage location the name was bound to at the time the closure was created.[b] A closure—unlike a plain function—allows the function to access those captured variables through the closure's reference to them, even when the function is invoked outside their scope.

--lexical scope(static scope)--
With lexical scope, a name always refers to its (more or less) local lexical environment. This is a property of the program text and is made independent of the runtime call stack by the language implementation. Because this matching only requires analysis of the static program text, this type of scoping is also called static scoping

--dynamic scope--
With dynamic scope, a global identifier refers to the identifier associated with the most recent environment, and is uncommon in modern languages.[4] In technical terms, this means that each identifier has a global stack of bindings. Introducing a local variable with name x pushes a binding onto the global x stack (which may have been empty), which is popped off when the control flow leaves the scope. Evaluating x in any context always yields the top binding. Note that this cannot be done at compile-time because the binding stack only exists at run-time, which is why this type of scoping is called dynamic scoping.

--anonymous function(common lambda expression)--
In computer programming, an anonymous function (also function literal or lambda abstraction) is a function definition that is not bound to an identifier. Anonymous functions are often:[1]
  -passed as arguments to higher-order functions, or
  -used to construct the result of a higher-order function that needs to return a function.

--Lambda calculus--
Lambda calculus (also written as λ-calculus) is a formal system in mathematical logic for expressing computation based on function abstraction and application using variable binding and substitution.

Lambda expressions are composed of
-variables v1, v2, ..., vn, ...
-the abstraction symbols lambda 'λ' and dot '.'
-parentheses ( )

The set of lambda expressions, Λ, can be defined inductively:
-If x is a variable, then x ∈ Λ
-If x is a variable and M ∈ Λ, then (λx.M) ∈ Λ
-If M, N ∈ Λ, then (M N) ∈ Λ
-Instances of rule 2 are known as abstractions and instances of rule 3 are known as applications.
```
