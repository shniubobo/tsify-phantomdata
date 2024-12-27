# tsify-phantomdata

To reproduce:

```
pnpm i
pnpm wasm-pack build --dev
```

This is the generated `pkg/tsify_phantomdata.d.ts`:

```typescript
/* tslint:disable */
/* eslint-disable */
export interface Foo {}
```

But the following is expected:

```typescript
/* tslint:disable */
/* eslint-disable */
export interface Foo<T> {}
```

Or:

```typescript
/* tslint:disable */
/* eslint-disable */
export interface Foo<_> {}
```
