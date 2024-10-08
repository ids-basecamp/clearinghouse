/*
 *  Copyright (c) 2023 truzzt GmbH
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License, Version 2.0 which is available at
 *  https://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Contributors:
 *       truzzt GmbH - Initial implementation
 *
 */
package de.truzzt.clearinghouse.edc.app.message;

import com.fasterxml.jackson.annotation.JsonIgnore;

public abstract class AbstractResponse {

    @JsonIgnore
    protected Integer httpStatus;

    public AbstractResponse() {
    }
    public AbstractResponse(int httpStatus) {
        this.httpStatus = httpStatus;
    }

    @JsonIgnore
    public boolean isSuccess() {
        return (httpStatus == null) || ((httpStatus >= 200) & (httpStatus <= 299));
    }

    @JsonIgnore
    public Integer getHttpStatus() {
        return httpStatus;
    }
}