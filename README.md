# tscale_sequence

## description

**[Chinese](README_CN.md)**

I accidentally discovered it when I encountered a need while playing a game. I'm not a mathematician. If you have any questions, I hope you can give me some advice.

Define a new sequence, called t-scale-sequence. It measures the relationship between a number of continuous factors. Below is an example of it.

> Assume that the scale t represents the year and a represents the population born in a certain year. From the current t we can infer the age group of a.
>
> Suppose that the impact of each age group on the birth population is a fixed coefficient. For example, the impact coefficient of a 32-year-old person on the birth population in the next year is 0.1. Assume that there are 100 32-year-olds now, then next year The birth population will be affected to 10 more people.
>
> Through the t-scale sequence, we can use the influence coefficient to roughly evaluate the relationship between the birth rate of the previous year and the birth rate of the next year.
>
> When the year tends to infinity (actually, there is an obvious ratio only after a few years, and it does not need to tend to infinity), there will be an obvious coefficient ratio between the number of people born in the previous year and the number of people born this year.

## Definition

The definition of the t-scale sequence is very simple. According to the definition, the Fibonacci sequence and the generalized Fibonacci sequence are both cases of the t-scale sequence.

#### Formula

Given any sequence, each time starting from the previous element, take elements equal to the length of the initial sequence, find their sum, and use this sum as the new value.

The initial sequence can take on any value. But it should be noted that the subsequent new value cannot be equal to 0 through calculation.

All betas must be greater than or equal to 0, and at least one beta must be greater than 0.

For the case where beta is equal to 1, it is called a standard t scale array.

$$\dim a_1,a_2,..,a_n \quad a\in R$$

$$a_t=\sum_{i=1}^n \beta_{i} a_{t-i} \quad t>n,a_t\not ={0}$$

$$\forall i, \ \beta_i \geq 0 \quad \text{and} \quad \exists i, \ \beta_i > 0$$

#### Properties

No matter what initial value you take, when the length of the sequence approaches infinity, the ratio of the previous element to the current element is a constant value, and this constant value is only affected by beta.

We can get the following theorem

$$\lim_{t \to +\infty} \frac{a_t}{a_{t-1}}= C$$

$$\sum \beta_i < 1 \implies 0 < C < 1$$

$$\sum \beta_i = 1 \implies C = 1$$

$$\sum \beta_i > 1 \implies C > 1$$

Continue the derivation and let r=C to get the following formula

$$ \lim_{t \to +\infty} \frac{a_t}{a_{t-1}}= r$$

$$\therefore \lim_{t \to +\infty} \frac{\sum_{i=1}^n \beta_i r^{n-i} a_{t-n}}{r^{n-1} a_{t-n}}=r$$

Simplifying when beta=1 we get:

$$when \quad \beta =1$$

$$r=2-r^{-n}$$

Simplifying when beta is not 1 we get:

$$\lim_{t \to +\infty} \sum_{i=1}^n \beta_i r^{1-i} a_{t-n}=r$$

Both simplifications can be solved by Newton's method to get the final answer