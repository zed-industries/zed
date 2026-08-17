# MathJax

Fourier Transform

\\[
\begin{aligned}
f(x) &= \int_{-\infty}^{\infty}F(s)(-1)^{ 2xs}ds \\\\
F(s) &= \int_{-\infty}^{\infty}f(x)(-1)^{-2xs}dx
\end{aligned}
\\]

The kernel can also be written as \\(e^{2i\pi xs}\\) which is more frequently used in literature.

> Proof that \\(e^{ix} = \cos x + i\sin x\\) a.k.a Euler's Formula:
>
> \\(
\begin{aligned}
  e^x &= \sum_{n=0}^\infty \frac{x^n}{n!} \implies e^{ix} = \sum_{n=0}^\infty \frac{(ix)^n}{n!} \\\\
  \cos x &= \sum_{m=0}^\infty \frac{(-1)^m x^{2m}}{(2m)!} = \sum_{m=0}^\infty \frac{(ix)^{2m}}{(2m)!} \\\\
  \sin x &= \sum_{s=0}^\infty \frac{(-1)^s x^{2s+1}}{(2s+1)!} = \sum_{s=0}^\infty \frac{(ix)^{2s+1}}{i(2s+1)!} \\\\
  \cos x + i\sin x &= \sum_{l=0}^\infty \frac{(ix)^{2l}}{(2l)!} + \sum_{s=0}^\infty \frac{(ix)^{2s+1}}{(2s+1)!} = \sum_{n=0}^\infty \frac{(ix)^{n}}{n!} \\\\
         &= e^{ix}
\end{aligned}
\\)
>


Pauli Matrices

\\[
\begin{aligned}
  \sigma_x &= \begin{pmatrix}
  1 & 0 \\\\ 0 & 1
  \end{pmatrix} \\\\
  \sigma_y &= \begin{pmatrix}
  0 & -i \\\\ i & 0
  \end{pmatrix} \\\\
  \sigma_z &= \begin{pmatrix}
  1 & 0 \\\\ 0 & -1
  \end{pmatrix}
\end{aligned}
\\]