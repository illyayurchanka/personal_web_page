---
title: Relativistic Quantum Mechanics
description: My short explanation of Relativistic Quantum Mechanics
pubDate: March 12 2026
heroImage: ../../assets/dirac_photo.webp
---

<!-- # Groups -> SU(2) + representation  -->
<!-- # Lorentz group + spinors  -->
<!-- # Electromagnetism and Dirac equation with em field  -->
## The Free Dirac Equation and it's Solution
### Intro

While Schrödinger equation revolutionised physics world, the same thing did Special Theory of Relativity by Albert Einstein. This theory postulated that law of motion are true in any inertia system if they are true in one, mathematically that meant that equation of motion should be Lorentz covariant. The problem was - Schrödinger equation was not.

That is why we must to find a proper relativistic quantum theory that will satisfy both postulates by non-relativistic QM and SR. 

Early attempts such as Klein-Gordon equation weren't succesfull but they gave us fundament which we can use in our theory. Dirac began his path with seeking a relativistically covariant equation of form 
$$
    i \hbar \frac{\partial \psi}{\partial t} = H\psi
$$
where the hamiltonian $H$ is a linear hermitian operator and it has no explicit time dependence for a closed physical system. 
Such an equation is linear in the time derivative, so it is natural to attempt to form a hamiltonian linear in the space derivatives as well.
$$
i \hbar \frac{\partial \psi}{\partial t} = \frac{\hbar c}{i}\left( \alpha_1 \frac{\partial \psi}{\partial x^1} +\alpha_2 \frac{\partial \psi}{\partial x^2} + \alpha_3 \frac{\partial \psi}{\partial x^3}\right) + \beta m c^2 \psi \equiv H \psi
\hspace{2em} \text{(1)}
$$
For the equation to be invariant, Dirac proposed that it be considered as a matrix equation. The wave function $\psi$, in analogy with the spin wave functions of non-relativistic quantum mechanics, is written as a column matrix with $N$ components:
$$
\psi =
\begin{bmatrix}
\psi_{1}\\
. \\
. \\
. \\

\psi_{N}
\end{bmatrix}
$$
and the constant coefficients $\alpha_i$, $\beta$ are $N \times N$ matrices. 

This equation must satisfy 3 conditions to serve as a satisfactory point of departure:
1. It must give the correct energy-momentum relation: $E^2 = p^2 c^2 + m^2 c^2$, for a free particle.
2. It must allow a continuity equation and a probability interpretation for the wave function $\psi$, as with classical QM.
3. IT MUST BE LORENTZ COVARIANT.

### First condition

As I said before it must give the correct energy-momentum relation. Previously I also said that there were unsuccessful attempts such as Klein-Gordon equation. But this attempt gave us important property: in order that the correct energy-momentum emerge from our equation, each component of $\psi$ must satisfy the Klein-Gordon second-order equation:
$$
 -\hbar ^2 \frac{ \partial ^2 \psi_{\sigma} }{ \partial t^2 } = (-\hbar^2 c^2 \nabla ^2 + m ^2 c^4)\psi_{\sigma}
$$
Iterating our equation we find:
$$
-\hbar ^ 2 \frac{\partial ^ 2 \psi}{\partial t^2} = \hbar ^2 c^2 \sum^3_{i,j = 1} \frac{\alpha_{j}\alpha_{i}+\alpha_{i}\alpha_{j}}{2} \frac{ \partial ^2 \psi }{ \partial x^i \partial x^j } + \frac{\hbar mc^3}{i}\sum^3_{i=1}(\alpha_{i}\beta + \beta\alpha_{i})\frac{ \partial \psi }{ \partial x^i } + \beta^2m^2c^4\psi 
$$ 
We may satisfy this equation if the four matrices $\alpha_{i}$, $\beta$ obey the algebra:
$$
\begin{matrix}
\alpha_{i}\alpha_{k} + \alpha_{k}\alpha_{i} = 2\delta_{ik} \\
\alpha_{i}\beta + \beta\alpha_{i} = 0 \\
\alpha_{i}^2 = \beta^2 = 1
\end{matrix}
$$
By expanding on those properties we can find that those matrices must be even-dimensional. The smallest possible even dimension $N=2$, is ruled out because it only can accommodate only the three mutually anticommuting Pauli matrices $\sigma_{i}$ plus a unit matrix. So the smallest possible dimension for $\alpha_{i}$ and $\beta$ is $N=4$. We can select following representation of matrices:
$$
\alpha_{i} = \begin{bmatrix}
0 & \sigma_{i} \\
\sigma_{i} & 0 
\end{bmatrix} \hspace{2em} \beta = \begin{bmatrix}
1 & 0 \\
0 & -1
\end{bmatrix}
$$

The equation $(1)$ with those parameters will satisfy our first condition.
### Second condition

For our second condition we must construct the differential law of current conservation. To do this we introduce the hermitian conjugate wave function $\psi ^{\dagger}=(\psi_{1}^*\dots \psi_{4}^*)$ and left-multiply $(1)$ by $\psi ^{\dagger}$. Than we form hermitian conjugate of $(1)$ and right-multiply by $\psi$. This will give us two equations which we can subtract (note that $\alpha=\alpha ^{\dagger}$ and $\beta=\beta ^{\dagger}$). We find:
$$
i\hbar \frac{ \partial  }{ \partial t } \psi ^{\dagger}\psi = \sum^3_{k=1} \frac{\hbar c}{i}\frac{ \partial  }{ \partial x^k } (\psi ^{\dagger}\alpha^k \psi)
$$
or
$$
\frac{ \partial  }{ \partial t } \rho + \text{div}\mathbf{j}=0 \hspace{2em}\text{(2)}
$$
where we make the identification of probability density:
$$
\rho = \psi ^{\dagger}\psi=\sum^4_{\sigma=1}\psi^*_{\sigma}\psi_{\sigma}
$$
and of a probability current with three components:
$$
j^k = c\psi ^{\dagger}\alpha^k\psi
$$
Integrating $(2)$ over all spaces and using Greens' theorem, we find:
$$
\frac{ \partial  }{ \partial t } \int d^3x\psi ^{\dagger}\psi=0
$$
which encourages the tentative interpretation of $\rho=\psi ^{\dagger}\psi$ as a positive definite probability density. 
### Third condition 


Our fist task is to create a set of observation of Dirac Equation made by observers $O$ and $O'$ in their respective references frames. That means we need to find a transformation law that allows $O'$ to compute $\psi'(x')$ if given $\psi(x)$. And this transformation law must lead to wave functions which are solutions of Dirac equation of the same form in the primed as well as unprimed reference frame. If this condition will be satisfied, we will fulfil Lorentz covariance.
Lets start with representing Dirac equation in a four-dimensional notation. We will multiply $(1)$ by $\frac{\beta}{c}$ and introduce notation:
$$
\gamma^0 = \beta \hspace{2em} \gamma^i = \beta \alpha_{i} \hspace{2em} i=1,2,3 
$$
This gives:
$$
(i\hbar\mathbf{\gamma}\nabla - mc)\psi=0
$$
Additionally we will introduce new notation:
$$
\not{A} = \gamma^\mu A_{\mu}=\gamma^0A^0 - \gamma \cdot \mathbf{A}
$$
where:
$$
\gamma^i = \begin{bmatrix}
0 & \sigma^i \\
-\sigma^i & 0
\end{bmatrix} \hspace{2em} \gamma^0 = \begin{bmatrix}
1  & 0 \\
0 & -1
\end{bmatrix}
$$
So Dirac equation takes form of:
$$ 
(i\hbar \not{\nabla} - mc)\psi=0 \hspace{2em} \text{(3)}
$$
Now, having proper notation, we can start our proof of covariance. We need to satisfy two requirements:
1. There must be an explicit prescription, which allows observer $O'$, given the $\psi(x)$ of observer $O$, to compute the $\psi'(x')$ which describes to $O'$ the same physical state.
2. According to relativity principle, $\psi'(x')$ will be a solution of an equation which takes the form of $(3)$ in the primed system:
$$
\left( i\hbar \bar{\gamma}^\mu \frac{ \partial  }{ \partial x^{\mu'} }- mc  \right) \psi'(x') = 0
$$
$\bar{\gamma}^\mu$ satisfy the anticommutation relations; therefore $\bar{\gamma}^{0{\dagger}} = \bar{\gamma}^0$ and $\bar{\gamma}^{i{\dagger}} = -\bar{\gamma}^i$ as required for hermitian hamiltonian.
It can be shown that all such $4\times 4$ matrices $\bar{\gamma}^\mu$ are equivalent up to a unitary transformation $U$:
$$
\bar{\gamma}_{\mu}=U^{\dagger}\gamma_{\mu}U \hspace{2em} U^{\dagger}=U^{-1}
$$

 <!-- # Free Dirac equation and solutions  -->
<!-- # Pauli eq + Klein Paradox -->
<!-- # Dirac Hydrogen Atom  -->
<!-- # $\vec{J}=\vec{L}+\vec{S}$ in Dirac equation  -->
<!-- # Classical fields (classical scalars and spinors) -->
<!-- # Canonical quantization of scalar field + particle interpretation  -->
<!-- # Scattering matrix $S$ -->
<!-- # Feynman rules for real and complex scalar + example -->
<!-- # *QED* -->
