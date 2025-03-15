# Clearinghouse Migration Guide

## Migration Steps v1.x to 2.0

### 1. Remove EDC
Since the EDC is no longer required, it can be safely decommissioned and removed from your deployment.

### 2. Verify Database Configuration
No database changes are required, but it is recommended to ensure that all connections remain functional after the migration.

### 3. Update Certificate Configuration
Ensure that the existing connector certificate is configured as the Clearinghouse signature certificate.

### 4. Update Incoming Connection Pointers
Modify any references to the old connection endpoint and point them to `ch-app`.

