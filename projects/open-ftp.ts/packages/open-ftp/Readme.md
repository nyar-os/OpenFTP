# `@wae/open-ftp`

Facade for the OpenFTP application.

- loads the platform native addon lazily
- exposes the typed capability, audit, CLI-contract, and GUI surface
- owns the structured `AppError` used by every consumer

The CLI locale catalogs live in [`@wae/open-ftp-cli`](../open-ftp-cli). This
package owns no `bin` and no locales.

The product `bin` lives in [`@wae/open-ftp-cli`](../open-ftp-cli). This package
must never expose a `bin` of its own.

```js
import { capabilities, invokeCapability } from '@wae/open-ftp';

const report = capabilities();
console.log(report.items.map((item) => [item.id, item.state]));

// Skeleton round: every domain capability refuses with `not_implemented`.
invokeCapability('endpoint-list');
```

See the repository `Readme.md` for build and test commands.
